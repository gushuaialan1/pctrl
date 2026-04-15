#!/usr/bin/env python3
"""
tts_pipeline.py

Example Python integration: calls pctrl CLI via subprocess and wraps the
result in a simple SSML-like phoneme tag.

Usage:
    python tts_pipeline.py "单于夜遁逃"
"""

import json
import subprocess
import sys


def pctrl_convert(text: str) -> dict:
    result = subprocess.run(
        ["pctrl", "convert", text, "--format", "json"],
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(result.stdout)


def to_ssml(data: dict) -> str:
    tokens = data.get("tokens", [])
    parts = []
    for token in tokens:
        surface = token.get("surface", "")
        pinyin = " ".join(token.get("pinyin", []))
        parts.append(
            f'<phoneme alphabet="pinyin" ph="{pinyin}">{surface}</phoneme>'
        )
    return f"<speak>{''.join(parts)}</speak>"


def main() -> None:
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} 'Chinese sentence'", file=sys.stderr)
        sys.exit(1)

    sentence = sys.argv[1]
    result = pctrl_convert(sentence)
    print(to_ssml(result))


if __name__ == "__main__":
    main()
