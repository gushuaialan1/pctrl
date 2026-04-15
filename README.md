# pctrl - Pronunciation Control

中文 TTS 预处理层，专注历史叙事/纪实类视频的多音字读音纠正。

## 特性

- **三层词典架构**：通用词层（约 10.5 万条）+ 历史专有词层（约 4,900 条）+ 人工纠正层
- **基于 CC-CEDICT**：利用开源中英词典的词组级拼音标注，覆盖人名、地名、官职、朝代等历史专有名词
- **前向最大匹配**：Trie 前缀树引擎，运行时零开销
- **多种输出格式**：plain、json、segmented，以及 Bert-VITS2 / GPT-SoVITS / generic_phoneme_json 导出

## 安装

```bash
cd /path/to/pctrl
cargo build --release
```

二进制位于 `target/release/pctrl`。

## 快速开始

```bash
# 单句转换
./target/release/pctrl convert "长孙无忌"
# 输出: zhang3 sun1 wu2 ji4

# JSON 输出
./target/release/pctrl convert "单于夜遁逃" --format json

# 分析决策路径
./target/release/pctrl analyze "唐朝的长安城"

# 校验词典
./target/release/pctrl dict validate dictionaries/history/history_core.json

# 健康检查
./target/release/pctrl doctor
```

## 项目架构

7-crate Rust workspace：

| crate | 职责 |
|-------|------|
| `pctrl-core` | 核心数据结构（DictionaryEntry、Token、PronunciationResult） |
| `pctrl-dict` | Trie 前缀树与词典加载 |
| `pctrl-segment` | CJK 感知切分（保留标点/数字） |
| `pctrl-engine` | 正向最大匹配引擎 + 单字 fallback |
| `pctrl-output` | 多格式输出 |
| `pctrl-config` | 分层 TOML 配置 |
| `pctrl-cli` | CLI 主程序 |

## 三层词典

```toml
[dictionaries]
enabled = ["cc_cedict_common", "cc_cedict_history", "history_core"]
```

1. **cc_cedict_common** (priority 700): 约 10.5 万条日常词组，来源于 CC-CEDICT
2. **cc_cedict_history** (priority 800): 约 4,900 条历史专有名词（人名、地名、官职等）
3. **history_core** (priority 900): 人工精校的高危误读词（如 仆射 pu2 ye4、单于 chan2 yu2）

## 文档

- 完整使用指南与集成说明请参见 [`docs/usage.md`](docs/usage.md)

## 示例

`examples/` 目录包含可直接运行的集成示例：

- [`examples/batch_convert.sh`](examples/batch_convert.sh) — 批量转换文本目录
- [`examples/tts_pipeline.py`](examples/tts_pipeline.py) — Python 调用 pctrl 并输出 SSML
- [`examples/add_custom_entry.py`](examples/add_custom_entry.py) — 向 history_core.json 添加自定义词条
- [`examples/filter_history_terms.py`](examples/filter_history_terms.py) — 检索内置词典中的拼音模式

## 数据来源

- 拼音词典基于 [CC-CEDICT](https://www.mdbg.net/chinese/dictionary?page=cc-cedict) （CC BY-SA 4.0）

## License

MIT
