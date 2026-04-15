#!/usr/bin/env python3
"""
add_custom_entry.py

Appends a custom DictionaryEntry to dictionaries/history/history_core.json.

Usage:
    python add_custom_entry.py "词语" "ci3 yu3" "history.custom"
"""

import json
import os
import sys
from typing import Any


def load_history_core(project_root: str) -> list[dict[str, Any]]:
    path = os.path.join(project_root, "dictionaries", "history", "history_core.json")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def save_history_core(project_root: str, entries: list[dict[str, Any]]) -> None:
    path = os.path.join(project_root, "dictionaries", "history", "history_core.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(entries, f, ensure_ascii=False, indent=2)
        f.write("\n")


def make_entry(word: str, pinyin: str, category: str) -> dict[str, Any]:
    pinyin_list = pinyin.strip().split()
    return {
        "word": word,
        "pinyin": pinyin_list,
        "category": category,
        "priority": 900,
        "source": "history_core",
        "common_errors": [],
        "notes": f"Added by {sys.argv[0]}",
        "enabled": True,
        "tags": ["custom"],
        "version": 1,
    }


def find_project_root() -> str:
    """Assume script lives in examples/ under the project root."""
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main() -> None:
    if len(sys.argv) < 3:
        print(f"Usage: {sys.argv[0]} <word> <pinyin> [category]", file=sys.stderr)
        print('Example: python add_custom_entry.py "词语" "ci3 yu3" "history.custom"', file=sys.stderr)
        sys.exit(1)

    word = sys.argv[1]
    pinyin = sys.argv[2]
    category = sys.argv[3] if len(sys.argv) > 3 else "history.custom"

    project_root = find_project_root()
    entries = load_history_core(project_root)

    # Prevent exact duplicate words
    for e in entries:
        if e.get("word") == word:
            print(f"Word '{word}' already exists in history_core.json. Skipping.")
            sys.exit(0)

    new_entry = make_entry(word, pinyin, category)
    entries.append(new_entry)
    save_history_core(project_root, entries)
    print(f"Added '{word}' ({pinyin}) to history_core.json.")


if __name__ == "__main__":
    main()
