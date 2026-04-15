# pctrl Usage Guide

Complete user documentation for integrating `pctrl` into TTS workflows.

---

## Installation

### Build from source

```bash
git clone https://github.com/gushuaialan1/pctrl.git
cd pctrl
cargo build --release
```

The compiled binary will be located at `target/release/pctrl`.

### Download prebuilt binaries

Visit the [GitHub Releases](https://github.com/gushuaialan1/pctrl/releases) page and download the binary for your platform.

### Add to PATH

```bash
# Linux / macOS
sudo cp target/release/pctrl /usr/local/bin/

# Or add to a local bin directory
mkdir -p ~/.local/bin
cp target/release/pctrl ~/.local/bin/
# Ensure ~/.local/bin is in your PATH
```

---

## Quick Start Examples

### Single sentence conversion

```bash
pctrl convert "长孙无忌"
# Output: zhang3 sun1 wu2 ji4
```

### JSON output with piping to jq

```bash
pctrl convert "单于夜遁逃" --format json | jq '.tokens[].pinyin'
```

### Batch processing a text file

Create `input.txt`:

```
唐朝的长安城
单于夜遁逃
```

Run:

```bash
pctrl convert --file input.txt --format json > output.json
```

### Processing an entire directory

Place multiple `.txt` files in `texts/` and run:

```bash
pctrl convert --dir texts --format plain > all_pinyin.txt
```

---

## CLI Command Reference

### `convert`

Converts Chinese text to pinyin using the configured dictionaries.

**Options:**
- `text`: Text to convert (positional)
- `--format <FORMAT>`: Output format (`plain`, `json`, `segmented`). Default: `plain`
- `--stdin`: Read input from stdin
- `--file <PATH>`: Read input from a file
- `--dir <PATH>`: Read all `.txt` files from a directory
- `--config <PATH>`: Use a custom config file

**Examples:**

```bash
pctrl convert "唐朝"
pctrl convert "唐朝" --format json
pctrl convert --file script.txt --format segmented
pctrl convert --dir transcripts/ --format plain
cat sentences.txt | pctrl convert --stdin --format json
```

### `analyze`

Shows the decision path for each token, including source dictionary, priority, and confidence.

**Options:**
- `text`: Text to analyze
- `--config <PATH>`: Use a custom config file

**Example:**

```bash
pctrl analyze "唐朝的长安城"
```

Output includes token-level details such as `surface`, `pinyin`, `source`, `strategy`, `priority`, and `confidence`.

### `init`

Creates a default configuration file.

**Options:**
- `--project`: Create config in `.pctrl/config.toml` (project-local)

**Examples:**

```bash
pctrl init              # Creates ~/.config/pctrl/config.toml
pctrl init --project    # Creates ./.pctrl/config.toml
```

### `doctor`

Performs a health check: loads the configuration and dictionaries, reports entry counts, and warns about duplicate words.

**Options:**
- `--config <PATH>`: Use a custom config file

**Example:**

```bash
pctrl doctor
```

### `dict validate`

Validates a custom dictionary JSON file.

**Example:**

```bash
pctrl dict validate dictionaries/history/history_core.json
```

Checks for empty words, empty pinyin, and suspicious pinyin strings (must end with tone number 1-5).

### `dict list`

Lists the currently enabled dictionaries from the configuration.

**Example:**

```bash
pctrl dict list
```

### `benchmark`

Measures conversion throughput for a text file.

**Options:**
- `path`: Path to text file
- `--config <PATH>`: Use a custom config file

**Example:**

```bash
pctrl benchmark script.txt
```

### `export`

Exports text to phoneme JSON formats compatible with TTS inference pipelines.

**Options:**
- `path`: Path to input text file
- `--format <FORMAT>`: Target format (`bert-vits2`, `gpt-sovits`, `generic_phoneme_json`)
- `--config <PATH>`: Use a custom config file

**Examples:**

```bash
pctrl export lyrics.txt --format bert-vits2
pctrl export lyrics.txt --format gpt-sovits
```

---

## Dictionary System Explained

### The 3-tier system

pctrl uses a layered dictionary architecture:

1. **`cc_cedict_common`** (priority 700): ~105,000 everyday words and phrases from CC-CEDICT.
2. **`cc_cedict_history`** (priority 800): ~4,900 history-specific terms (names, places, titles, dynasties).
3. **`history_core`** (priority 900): Hand-curated high-risk misreadings (e.g. 仆射 `pu2 ye4`, 单于 `chan2 yu2`).

The default configuration enables all three:

```toml
[dictionaries]
enabled = ["cc_cedict_common", "cc_cedict_history", "history_core"]
```

### How priority works

The engine uses forward maximum matching with a Trie. When multiple dictionary entries match the same text position, the winner is decided by:

1. **Longer word wins first**
2. **If length is equal, higher `priority` wins**

For example, if both `长安` and `长安城` match, `长安城` (3 characters) is chosen over `长安` (2 characters). If two entries have the same length, the one with the larger `priority` value is selected.

### Adding custom entries to `history_core.json`

The `history_core.json` file is the recommended place for user overrides. Each entry follows the `DictionaryEntry` structure:

```json
{
  "word": "词语",
  "pinyin": ["ci3", "yu3"],
  "category": "history.custom",
  "priority": 900,
  "source": "history_core",
  "common_errors": [],
  "notes": "Optional note",
  "enabled": true,
  "tags": ["custom"],
  "version": 1
}
```

After editing, always validate:

```bash
pctrl dict validate dictionaries/history/history_core.json
```

### Validating custom dictionaries

Run `pctrl dict validate` on any JSON dictionary file before deploying it. It catches:

- Empty `word` fields
- Empty `pinyin` arrays
- Pinyin that does not end with a tone number (1-5)

---

## Integration Guides

### Bert-VITS2

Export your script to Bert-VITS2 phoneme JSON:

```bash
pctrl export --format bert-vits2 input.txt output.json
```

Each line in `input.txt` becomes a JSON object with `text` and `items` (each item has `text` and `phonemes`).

### GPT-SoVITS

Export to GPT-SoVITS compatible format:

```bash
pctrl export --format gpt-sovits input.txt output.json
```

The output structure is identical to Bert-VITS2 export; consume it in your GPT-SoVITS preprocessing pipeline.

### Python integration

Call `pctrl` from Python using `subprocess`:

```python
import subprocess
import json

def pctrl_convert(text: str) -> dict:
    result = subprocess.run(
        ["pctrl", "convert", text, "--format", "json"],
        capture_output=True, text=True, check=True
    )
    return json.loads(result.stdout)

data = pctrl_convert("单于夜遁逃")
print(data["tokens"][0]["pinyin"])
```

See `examples/tts_pipeline.py` for a complete SSML wrapper script.

### Shell pipeline

Batch-process files in a loop:

```bash
mkdir -p output
for f in texts/*.txt; do
    basename=$(basename "$f" .txt)
    pctrl convert --file "$f" --format json > "output/${basename}.json"
    echo "Processed: $basename"
done
```

See `examples/batch_convert.sh` for a polished version with progress reporting.

---

## Troubleshooting

### "Word not converted correctly"

1. Run `pctrl analyze "your sentence"` to see which dictionary source and priority was used.
2. Check if your custom dictionary is valid with `pctrl dict validate <path>`.
3. Ensure the dictionary is listed in `enabled` in your config.

### "Missing pinyin for character"

If a character falls back to itself instead of pinyin, the built-in PHF map does not contain it. Consider adding the character (or a multi-character word containing it) to `history_core.json`.

### Performance tips for large batches

- Use `--file` or `--dir` instead of invoking `pctrl` once per sentence.
- For very large files, run `pctrl benchmark your_file.txt` to estimate throughput.
- The engine uses a zero-runtime-overhead Trie; typical throughput is thousands of lines per second.
