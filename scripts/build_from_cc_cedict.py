#!/usr/bin/env python3
"""
CC-CEDICT pipeline for pctrl.
One-pass parse -> dedup -> Tier1 override -> generate PHF + history dict.
"""

import gzip
import json
import re
import sys
from pathlib import Path
from collections import defaultdict
from urllib.request import urlretrieve

CEDICT_URL = "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz"
CEDICT_CACHE = Path("/tmp/cedict_1_0_ts_utf-8_mdbg.txt.gz")
CEDICT_TXT = Path("/tmp/cedict_1_0_ts_utf-8_mdbg.txt")

# Tier 1: high-frequency function words and common polyphones.
# These override CC-CEDICT's first-reading heuristic to avoid absurd fallbacks
# like 的 -> di4, 了 -> liao3, 地 -> di4, etc.
TIER1_FALLBACK = {
    # 助词 / 语气词
    "的": "de5",
    "了": "le5",
    "着": "zhe5",
    "地": "de5",
    "得": "de5",
    "过": "guo5",
    "呢": "ne5",
    "吧": "ba5",
    "吗": "ma5",
    "啊": "a5",
    "哇": "wa5",
    "哪": "na5",
    "哦": "o5",
    "哟": "yo5",
    "呗": "bei5",
    "嘛": "ma5",
    "哩": "li5",
    "呐": "na5",
    "乎": "hu5",
    # 高频介词 / 连词 / 代词
    "和": "he2",
    "与": "yu3",
    "跟": "gen1",
    "同": "tong2",
    "被": "bei4",
    "把": "ba3",
    "将": "jiang1",
    "从": "cong2",
    "向": "xiang4",
    "对": "dui4",
    "给": "gei3",
    "让": "rang4",
    "在": "zai4",
    "于": "yu2",
    "以": "yi3",
    "由": "you2",
    "比": "bi3",
    "为": "wei4",
    "因": "yin1",
    "而": "er2",
    "但": "dan4",
    "或": "huo4",
    "若": "ruo4",
    "虽": "sui1",
    # 高频副词 / 否定词
    "都": "dou1",
    "也": "ye3",
    "还": "hai2",
    "就": "jiu4",
    "又": "you4",
    "很": "hen3",
    "更": "geng4",
    "最": "zui4",
    "太": "tai4",
    "不": "bu4",
    "没": "mei2",
    "别": "bie2",
    "无": "wu2",
    "可": "ke3",
    "能": "neng2",
    "会": "hui4",
    "要": "yao4",
    "应": "ying1",
    "该": "gai1",
    # 常见多音字（最高频读音优先）
    "长": "chang2",
    "行": "xing2",
    "重": "zhong4",
    "好": "hao3",
    "便": "bian4",
    "传": "chuan2",
    "分": "fen1",
    "干": "gan1",
    "降": "jiang4",
    "角": "jiao3",
    "结": "jie2",
    "劲": "jin4",
    "乐": "le4",
    "量": "liang4",
    "难": "nan2",
    "强": "qiang2",
    "少": "shao3",
    "数": "shu4",
    "相": "xiang1",
    "压": "ya1",
    "正": "zheng4",
    "中": "zhong1",
    "转": "zhuan3",
    "作": "zuo4",
    "朝": "chao2",
    "处": "chu3",
    "创": "chuang4",
    "斗": "dou4",
    "恶": "e4",
    "发": "fa1",
    "服": "fu2",
    "父": "fu4",
    "冠": "guan1",
    "观": "guan1",
    "荷": "he2",
    "吓": "xia4",
    "横": "heng2",
    "划": "hua4",
    "教": "jiao1",
    "尽": "jin3",
    "卡": "ka3",
    "看": "kan4",
    "空": "kong1",
    "落": "luo4",
    "脉": "mai4",
    "模": "mo2",
    "曲": "qu1",
    "任": "ren4",
    "散": "san4",
    "色": "se4",
    "识": "shi2",
    "说": "shuo1",
    "调": "tiao2",
    "万": "wan4",
    "血": "xie3",
    "旋": "xuan2",
    "咽": "yan1",
    "约": "yue1",
    "只": "zhi3",
    "种": "zhong3",
}


def ensure_cedict() -> Path:
    if not CEDICT_TXT.exists():
        if not CEDICT_CACHE.exists():
            print(f"Downloading CC-CEDICT from {CEDICT_URL} ...")
            urlretrieve(CEDICT_URL, CEDICT_CACHE)
            print("Download complete.")
        print(f"Decompressing {CEDICT_CACHE} ...")
        with gzip.open(CEDICT_CACHE, "rt", encoding="utf-8") as fin, open(
            CEDICT_TXT, "w", encoding="utf-8"
        ) as fout:
            fout.write(fin.read())
        print("Decompression complete.")
    return CEDICT_TXT


def parse_line(line: str):
    line = line.strip()
    if not line or line.startswith("#"):
        return None
    m = re.match(r"^(\S+)\s+(\S+)\s+\[(.*?)\]\s+/(.*)/$", line)
    if not m:
        return None
    trad, simp, pinyin_raw, defs_raw = m.groups()
    pinyins = pinyin_raw.split()
    defs = [d.strip() for d in defs_raw.split("/") if d.strip()]
    return {
        "traditional": trad,
        "simplified": simp,
        "pinyin": pinyins,
        "definitions": defs,
    }


def pinyin_to_tone3(py: str) -> str:
    py_lower = py.lower().strip()
    if not py_lower:
        return "?5"
    last = py_lower[-1]
    if last.isdigit() and last in "12345":
        base = py_lower[:-1].replace("u:", "v").replace(":", "")
        return base + last

    tone_marks = {
        "\u0101": ("a", 1),
        "\u00e1": ("a", 2),
        "\u01ce": ("a", 3),
        "\u00e0": ("a", 4),
        "\u0113": ("e", 1),
        "\u00e9": ("e", 2),
        "\u011b": ("e", 3),
        "\u00e8": ("e", 4),
        "\u012b": ("i", 1),
        "\u00ed": ("i", 2),
        "\u01d0": ("i", 3),
        "\u00ec": ("i", 4),
        "\u014d": ("o", 1),
        "\u00f3": ("o", 2),
        "\u01d2": ("o", 3),
        "\u00f2": ("o", 4),
        "\u016b": ("u", 1),
        "\u00fa": ("u", 2),
        "\u01d4": ("u", 3),
        "\u00f9": ("u", 4),
        "\u01d6": ("\u00fc", 1),
        "\u01d8": ("\u00fc", 2),
        "\u01da": ("\u00fc", 3),
        "\u01dc": ("\u00fc", 4),
    }
    tone = 5
    out_chars = []
    for ch in py_lower:
        if ch in tone_marks:
            base, tone = tone_marks[ch]
            out_chars.append(base)
        elif ch == "\u00fc":
            out_chars.append("\u00fc")
        elif ch.isalpha():
            out_chars.append(ch)
    return "".join(out_chars) + str(tone)


def is_cjk_char(ch: str) -> bool:
    cp = ord(ch)
    return (
        (0x4E00 <= cp <= 0x9FFF)
        or (0x3400 <= cp <= 0x4DBF)
        or (0x20000 <= cp <= 0x2A6DF)
        or (0x2A700 <= cp <= 0x2B73F)
        or (0x2B740 <= cp <= 0x2B81F)
    )


def is_history_term(entry: dict) -> bool:
    defs = " ".join(entry["definitions"]).lower()
    word = entry["simplified"]

    if not all(is_cjk_char(c) or c == "\u00b7" for c in word):
        return False

    strict_keywords = (
        "dynasty",
        "emperor",
        "empress",
        "king",
        "queen",
        "prince",
        "princess",
        "general",
        "minister",
        "official",
        "chancellor",
        "premier",
        "tribe",
        "ethnic group",
        "nomadic",
        "khanate",
        "han dynasty",
        "tang dynasty",
        "song dynasty",
        "ming dynasty",
        "qing dynasty",
        "zhou dynasty",
        "qin dynasty",
        "three kingdoms",
        "spring and autumn",
        "warring states",
        "surname",
        "family name",
        "clan name",
        "temple name",
        "posthumous title",
        "era name",
        "reign title",
        "commandery",
        "protectorate",
        "prefecture",
        "county seat",
        "capital of",
        "ancient city",
        "ancient town",
    )

    modern_exclusions = (
        "born between",
        "gen z",
        "internet slang",
        "loanword",
        "abbr. for",
        "japan",
        "anime",
        "manga",
        "k-pop",
        "slang",
        "threesome",
        "bye-bye",
        "authorized full-service",
    )

    has_history = any(k in defs for k in strict_keywords)
    is_modern = any(k in defs for k in modern_exclusions)
    return has_history and not is_modern


def entry_to_pctrl(
    entry: dict,
    source: str = "cc_cedict",
    category: str = None,
    priority: int = 800,
    tags: list = None,
) -> dict:
    word = entry["simplified"]
    pinyin = [pinyin_to_tone3(p) for p in entry["pinyin"]]
    defs = " ".join(entry["definitions"])
    dl = defs.lower()

    if category is None:
        if any(k in dl for k in ("surname", "family name", "clan")):
            category = "history.person.surname"
        elif any(k in dl for k in ("emperor", "king", "queen", "prince", "princess")):
            category = "history.title"
        elif any(k in dl for k in ("general", "minister", "official")):
            category = "history.office"
        elif any(k in dl for k in ("tribe", "ethnic", "nomadic")):
            category = "history.ethnic"
        elif "place" in dl or any(
            k in dl for k in ("capital", "province", "prefecture", "county")
        ):
            category = "history.place"
        elif any(k in dl for k in ("dynasty", "historical", "ancient", "reign")):
            category = "history.dynasty"
        else:
            category = "common"

    if tags is None:
        tags = ["history", "cc_cedict"]

    return {
        "word": word,
        "pinyin": pinyin,
        "category": category,
        "priority": priority,
        "source": source,
        "common_errors": [],
        "notes": " | ".join(entry["definitions"])[:200],
        "enabled": True,
        "tags": tags,
        "version": 1,
    }


def build_phf_map(char_map: dict, output_path: Path):
    cjk_map = {k: v for k, v in char_map.items() if is_cjk_char(k)}
    lines = [
        "use phf::phf_map;",
        "",
        "pub static PINYIN_MAP: phf::Map<char, &'static str> = phf_map! {",
    ]
    for ch, py in sorted(cjk_map.items(), key=lambda x: ord(x[0])):
        if ch == "'":
            ch_escaped = "\\'"
        elif ch == "\\":
            ch_escaped = "\\\\"
        else:
            ch_escaped = ch
        lines.append(f'    \'{ch_escaped}\' => "{py}",')
    lines.append("};")
    lines.append("")
    output_path.write_text("\n".join(lines), encoding="utf-8")


def main():
    cedict_txt = ensure_cedict()
    out_dir = Path(__file__).resolve().parent.parent / "dictionaries" / "history"
    out_dir.mkdir(parents=True, exist_ok=True)
    common_out_dir = Path(__file__).resolve().parent.parent / "dictionaries"
    common_out_dir.mkdir(parents=True, exist_ok=True)
    phf_path = (
        Path(__file__).resolve().parent.parent
        / "crates"
        / "pctrl-engine"
        / "src"
        / "pinyin_map_phf.rs"
    )

    char_candidates = defaultdict(list)
    history_entries = []
    common_entries = []
    seen_words = set()
    total_entries = 0

    print("Parsing CC-CEDICT (one-pass) ...")
    with open(cedict_txt, "r", encoding="utf-8") as f:
        for line in f:
            parsed = parse_line(line)
            if not parsed:
                continue
            total_entries += 1
            word = parsed["simplified"]
            pinyins = parsed["pinyin"]

            if len(word) == 1:
                py = pinyin_to_tone3(pinyins[0])
                char_candidates[word].append(py)
            elif len(word) >= 2 and word not in seen_words:
                seen_words.add(word)
                if is_history_term(parsed):
                    history_entries.append(entry_to_pctrl(parsed))
                else:
                    common_entries.append(
                        entry_to_pctrl(
                            parsed,
                            source="cc_cedict_common",
                            category="common",
                            priority=700,
                            tags=["common", "cc_cedict"],
                        )
                    )

    # Resolve single-char fallback map
    final_char_map = {}
    for ch, py_list in char_candidates.items():
        if ch in TIER1_FALLBACK:
            final_char_map[ch] = TIER1_FALLBACK[ch]
        else:
            # Deduplicate preserving order
            seen = set()
            ordered = []
            for py in py_list:
                if py not in seen:
                    seen.add(py)
                    ordered.append(py)
            final_char_map[ch] = ordered[0]

    # Write outputs
    history_path = out_dir / "cc_cedict_history.json"
    history_path.write_text(
        json.dumps(history_entries, ensure_ascii=False, indent=2), encoding="utf-8"
    )
    common_path = common_out_dir / "cc_cedict_common.json"
    common_path.write_text(
        json.dumps(common_entries, ensure_ascii=False, indent=2), encoding="utf-8"
    )
    build_phf_map(final_char_map, phf_path)

    print(f"Total parsed: {total_entries}")
    print(f"History entries: {len(history_entries)}")
    print(f"Common entries: {len(common_entries)}")
    print(f"Single-char fallback map: {len(final_char_map)} chars")
    print(f"Outputs:")
    print(f"  {history_path}")
    print(f"  {common_path}")
    print(f"  {phf_path}")


if __name__ == "__main__":
    main()
