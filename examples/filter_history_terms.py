#!/usr/bin/env python3
"""
filter_history_terms.py

Reads dictionaries/cc_cedict_common.json and filters entries where pinyin
contains user-specified patterns. Demonstrates inspecting built-in dictionaries.

Usage:
    python filter_history_terms.py "zhang" "chang"
"""

import json
import os
import sys


def load_dict(project_root: str, filename: str) -> list[dict]:
    path = os.path.join(project_root, "dictionaries", filename)
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def find_project_root() -> str:
    """Assume script lives in examples/ under the project root."""
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main() -> None:
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <pattern1> [pattern2] ...", file=sys.stderr)
        print('Example: python filter_history_terms.py "zhang3" "chang2"', file=sys.stderr)
        sys.exit(1)

    patterns = sys.argv[1:]
    project_root = find_project_root()
    entries = load_dict(project_root, "cc_cedict_common.json")

    matched = 0
    for entry in entries:
        pinyins = entry.get("pinyin", [])
        pinyin_str = " ".join(pinyins)
        if any(p.lower() in pinyin_str.lower() for p in patterns):
            print(f"{entry['word']} | {pinyin_str} | source={entry.get('source', 'unknown')}")
            matched += 1
            if matched >= 20:
                print("\n... (showing first 20 matches)")
                break

    print(f"\nTotal matches: {matched}")


if __name__ == "__main__":
    main()
