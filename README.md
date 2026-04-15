# pctrl (Pronunciation Control)

中文 TTS 预处理层，专注历史叙事与纪实类视频中的多音字读音纠正。
A Chinese TTS preprocessing layer focused on correcting polyphone pronunciations in historical narration and documentary videos.

![CI](https://github.com/gushuaialan1/pctrl/workflows/CI/badge.svg)
![GitHub Release](https://img.shields.io/github/v/release/gushuaialan1/pctrl)
![License](https://img.shields.io/github/license/gushuaialan1/pctrl)

## 特性 / Features

- **3-tier dictionary system** / 三层词典体系：通用层约 10.5 万条（CC-CEDICT）+ 历史专有词层约 4,900 条 + 人工精校层
- **Forward maximum-match Trie engine** / 前向最大匹配 Trie 引擎：运行时零开销，高效切分与查询
- **Multiple output formats** / 多种输出格式：plain、json、segmented，以及 Bert-VITS2 / GPT-SoVITS / generic 导出
- **Cross-platform CLI** / 跨平台 CLI：支持批量处理、内置基准测试与健康检查工具

## 快速开始 / Quick Start

下载预编译二进制文件，或通过源码构建：
Download a prebuilt binary, or build from source:

```bash
cd /path/to/pctrl
cargo build --release
```

单句转换示例：
Single-sentence conversion:

```bash
./pctrl convert "单于夜遁逃"
# chan2 yu2 ye4 dun4 tao2
```

JSON 输出示例：
JSON output:

```bash
./pctrl convert "长孙无忌" --format json
```

## 演示 / Demo

```bash
$ pctrl analyze "唐朝的长安城"
tang2  chao2  de5  chang2  an1  cheng2
```

典型高危多音字纠正示例：
Examples of high-risk polyphone corrections:

| 文本 / Text | 常见误读 / Common Misreading | 纠正读音 / Corrected Pinyin |
|-------------|------------------------------|----------------------------|
| 仆射 | pu2 she4 | pu2 ye4 |
| 单于 | dan1 yu2 | chan2 yu2 |
| 长孙 | zhang3 sun1 | zhang3 sun1 |

## 架构与技术栈 / Architecture & Tech Stack

本项目采用 7-crate Rust workspace 架构：
This project is organized as a 7-crate Rust workspace:

| crate | 职责 / Responsibility |
|-------|----------------------|
| `pctrl-core` | 核心数据结构（DictionaryEntry、Token、PronunciationResult） |
| `pctrl-dict` | Trie 前缀树构建与词典加载 |
| `pctrl-segment` | CJK 感知文本切分（保留标点与数字） |
| `pctrl-engine` | 前向最大匹配引擎 + 单字 fallback |
| `pctrl-output` | 多格式输出实现 |
| `pctrl-config` | 分层 TOML 配置管理 |
| `pctrl-cli` | CLI 主程序与命令解析 |

数据流示意：
Data flow:

```
Input Text -> Segment -> Trie Lookup (pctrl-dict) -> Engine (pick_best) -> Output Formatter
```

核心依赖：
Key dependencies:

- `clap` — 命令行参数解析
- `serde` / `serde_json` — 序列化与配置解析
- `phf` — 编译时完美哈希
- `toml` — TOML 配置文件处理

## 安装 / Installation

### 预编译二进制 / Prebuilt Binaries

从 GitHub Releases 下载适用于 Linux、macOS 或 Windows 的二进制文件。
Download from GitHub Releases for Linux, macOS, or Windows.

### 从源码构建 / Build from Source

```bash
git clone https://github.com/gushuaialan1/pctrl.git
cd pctrl
cargo build --release
```

构建完成后，二进制文件位于 `target/release/pctrl`。建议将其加入 PATH。
The binary will be at `target/release/pctrl`. Optionally add it to your PATH.

## 使用说明 / Usage

| 子命令 / Subcommand | 说明 / Description | 示例 / Example |
|---------------------|-------------------|----------------|
| `convert` | 将中文文本转换为拼音 / Convert Chinese text to pinyin | `pctrl convert "文本"` |
| `analyze` | 分析并输出决策路径 / Analyze and print decision path | `pctrl analyze "长安城"` |
| `init` | 初始化本地配置文件 / Initialize local configuration | `pctrl init` |
| `doctor` | 运行健康检查 / Run health checks | `pctrl doctor` |
| `dict validate` | 校验词典 JSON 文件 / Validate a dictionary JSON file | `pctrl dict validate dictionaries/history/history_core.json` |
| `dict list` | 列出已加载的词典 / List loaded dictionaries | `pctrl dict list` |
| `benchmark` | 执行性能基准测试 / Run performance benchmarks | `pctrl benchmark` |
| `export` | 导出为 TTS 格式 / Export to TTS formats | `pctrl export --format bert_vits2` |

## 配置 / Configuration

pctrl 使用三层词典优先级体系，可在 TOML 配置中启用：
pctrl uses a 3-tier dictionary priority system, configurable via TOML:

```toml
[dictionaries]
enabled = ["cc_cedict_common", "cc_cedict_history", "history_core"]
```

1. **cc_cedict_common** (priority 700): 约 10.5 万条日常词组，来源于 CC-CEDICT
2. **cc_cedict_history** (priority 800): 约 4,900 条历史专有名词（人名、地名、官职等）
3. **history_core** (priority 900): 人工精校的高危误读词

配置文件搜索路径（按优先级降序）：
Configuration file lookup paths (highest to lowest priority):

- `./.pctrl/config.toml`（项目级配置 / Project-level config）
- `~/.config/pctrl/config.toml`（用户级配置 / User-level config）

## 开发 / Development

运行测试、静态检查与构建：
Run tests, lints, and build:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release
```

仓库结构为 `crates/` 下的 7-crate workspace，示例脚本位于 `examples/`。
The repo is a 7-crate workspace under `crates/`, with example scripts in `examples/`.

## 路线图 / Roadmap

- **Phase 1 (Done)** / 第一阶段（已完成）: 核心引擎、Trie 实现、CLI、CC-CEDICT 集成、CI/CD
- **Phase 2 (Next)** / 第二阶段（下一步）: 上下文感知消歧规则、HTTP API 服务
- **Phase 3 (Future)** / 第三阶段（未来）: SSML 标签注入、实时流式处理模式

## 文档与示例 / Documentation & Examples

- 完整使用指南请参见 [`docs/usage.md`](docs/usage.md)
  Full usage guide: [`docs/usage.md`](docs/usage.md)

`examples/` 目录包含以下可直接运行的示例脚本：
The `examples/` directory contains the following runnable scripts:

- [`examples/batch_convert.sh`](examples/batch_convert.sh) — 批量转换文本目录 / Batch convert a directory of text files
- [`examples/tts_pipeline.py`](examples/tts_pipeline.py) — Python 调用 pctrl 并输出 SSML / Python wrapper calling pctrl and emitting SSML
- [`examples/add_custom_entry.py`](examples/add_custom_entry.py) — 向 history_core.json 添加自定义词条 / Add a custom entry to history_core.json
- [`examples/filter_history_terms.py`](examples/filter_history_terms.py) — 检索内置词典中的拼音模式 / Search pinyin patterns in built-in dictionaries

## 许可证与致谢 / License & Credits

- 代码许可证 / Code license: [MIT](LICENSE)
- 词典数据基于 / Dictionary data based on: [CC-CEDICT](https://www.mdbg.net/chinese/dictionary?page=cc-cedict)（CC BY-SA 4.0）
