use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use pctrl_config::Config;
use pctrl_dict::Dictionary;
use pctrl_engine::Engine;
use pctrl_output::{format_result, OutputFormat};

#[derive(Parser)]
#[command(name = "pctrl")]
#[command(about = "TTS Pronunciation Control CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        text: Option<String>,
        #[arg(short, long, default_value = "plain")]
        format: String,
        #[arg(long)]
        stdin: bool,
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(short, long)]
        dir: Option<PathBuf>,
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Analyze {
        text: String,
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Init {
        #[arg(long)]
        project: bool,
    },
    Doctor {
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Dict {
        #[command(subcommand)]
        action: DictAction,
    },
    Benchmark {
        path: PathBuf,
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Export {
        path: PathBuf,
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum DictAction {
    List,
    Validate { path: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert {
            text,
            format,
            stdin,
            file,
            dir,
            config,
        } => {
            let inputs = gather_inputs(text, stdin, file, dir)?;
            if inputs.is_empty() {
                anyhow::bail!("Please provide text, --stdin, --file, or --dir");
            }

            let config = Config::load(config.as_deref())?;
            let dict = load_dictionary(&config)?;
            let engine = Engine::new(dict);
            let fmt = OutputFormat::from_str(&format)?;

            for (idx, input) in inputs.iter().enumerate() {
                let result = engine.convert(input);
                let out = format_result(&result, fmt)?;
                if inputs.len() > 1 && fmt == OutputFormat::Json {
                    if idx == 0 {
                        println!("[");
                    }
                    print!("{}", out);
                    if idx + 1 < inputs.len() {
                        println!(",");
                    } else {
                        println!();
                        println!("]");
                    }
                } else {
                    println!("{}", out);
                }
            }
        }
        Commands::Analyze { text, config } => {
            let config = Config::load(config.as_deref())?;
            let dict = load_dictionary(&config)?;
            let engine = Engine::new(dict);
            let result = engine.convert(&text);
            println!("Input: {}", result.text);
            println!("Tokens:");
            for (i, token) in result.tokens.iter().enumerate() {
                println!(
                    "  [{}] surface={} pinyin={:?} source={} strategy={} priority={} confidence={}",
                    i,
                    token.surface,
                    token.pinyin,
                    token.source,
                    token.strategy,
                    token.priority,
                    token.confidence
                );
            }
        }
        Commands::Init { project } => {
            let path = if project {
                fs::create_dir_all(".pctrl")?;
                PathBuf::from(".pctrl/config.toml")
            } else {
                let home = dirs::config_dir().context("Could not find config dir")?;
                fs::create_dir_all(home.join("pctrl"))?;
                home.join("pctrl/config.toml")
            };
            if path.exists() {
                println!("Config already exists at {}", path.display());
            } else {
                fs::write(&path, Config::default_toml())?;
                println!("Initialized config at {}", path.display());
            }
        }
        Commands::Doctor { config } => {
            let config = Config::load(config.as_deref())?;
            println!("Configuration loaded successfully");
            println!("Enabled dictionaries: {:?}", config.dictionaries.enabled);
            let d = load_dictionary(&config)?;
            println!("Dictionary loaded: {} entries", d.entries.len());
            let mut seen = std::collections::HashSet::new();
            let mut duplicates = Vec::new();
            for e in &d.entries {
                if !seen.insert(&e.word) {
                    duplicates.push(&e.word);
                }
            }
            if !duplicates.is_empty() {
                println!("Warning: duplicate words found: {:?}", duplicates);
            }
        }
        Commands::Dict { action } => match action {
            DictAction::List => {
                let config = Config::load(None)?;
                println!("Enabled dictionaries: {:?}", config.dictionaries.enabled);
            }
            DictAction::Validate { path } => {
                validate_dictionary(&path)?;
            }
        },
        Commands::Benchmark { path, config } => {
            let content = fs::read_to_string(&path)?;
            let lines: Vec<&str> = content.lines().collect();
            let config = Config::load(config.as_deref())?;
            let dict = load_dictionary(&config)?;
            let engine = Engine::new(dict);

            let start = Instant::now();
            for line in &lines {
                let _ = engine.convert(line);
            }
            let elapsed = start.elapsed();
            println!("Lines: {}", lines.len());
            println!("Total time: {:?}", elapsed);
            if elapsed.as_secs_f64() > 0.0 {
                println!(
                    "Throughput: {:.2} lines/sec",
                    lines.len() as f64 / elapsed.as_secs_f64()
                );
            }
        }
        Commands::Export {
            path,
            format,
            config,
        } => {
            let content = fs::read_to_string(&path)?;
            let lines: Vec<&str> = content.lines().collect();
            let config = Config::load(config.as_deref())?;
            let dict = load_dictionary(&config)?;
            let engine = Engine::new(dict);

            match format.as_str() {
                "bert-vits2" | "gpt-sovits" | "generic_phoneme_json" => {
                    let mut results = Vec::new();
                    for line in &lines {
                        let result = engine.convert(line);
                        let mut items = Vec::new();
                        for token in &result.tokens {
                            items.push(serde_json::json!({
                                "text": token.surface,
                                "phonemes": token.pinyin,
                            }));
                        }
                        results.push(serde_json::json!({
                            "text": result.text,
                            "items": items,
                        }));
                    }
                    println!("{}", serde_json::to_string_pretty(&results)?);
                }
                _ => anyhow::bail!("Unsupported export format: {}", format),
            }
        }
    }
    Ok(())
}

fn gather_inputs(
    text: Option<String>,
    stdin: bool,
    file: Option<PathBuf>,
    dir: Option<PathBuf>,
) -> Result<Vec<String>> {
    let mut inputs = Vec::new();
    if stdin {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        for line in buf.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                inputs.push(trimmed.to_string());
            }
        }
    }
    if let Some(f) = file {
        let content = fs::read_to_string(f)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                inputs.push(trimmed.to_string());
            }
        }
    }
    if let Some(d) = dir {
        for entry in fs::read_dir(d)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let content = fs::read_to_string(path)?;
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        inputs.push(trimmed.to_string());
                    }
                }
            }
        }
    }
    if let Some(t) = text {
        inputs.push(t);
    }
    Ok(inputs)
}

fn load_dictionary(config: &Config) -> Result<Dictionary> {
    let exe_dir = std::env::current_exe()?
        .parent()
        .context("no exe parent")?
        .to_path_buf();
    let project_dir = std::env::current_dir()?;

    let mut candidates = vec![
        exe_dir.join("dictionaries"),
        exe_dir.join("../../dictionaries"),
        project_dir.join("dictionaries"),
        project_dir.join("../dictionaries"),
    ];
    if let Ok(env_dict_dir) = std::env::var("PCTRL_DICT_DIR") {
        candidates.push(PathBuf::from(env_dict_dir));
    }
    for p in &config.dictionaries.paths {
        candidates.push(p.clone());
    }

    let mut all_entries = Vec::new();
    for source in &config.dictionaries.enabled {
        let mut found = false;
        for base in &candidates {
            let path = base.join(format!("history/{}.json", source));
            if path.exists() {
                let dict = Dictionary::load_json_file(&path, source)?;
                all_entries.extend(dict.entries);
                found = true;
                break;
            }
            let path2 = base.join(format!("{}.json", source));
            if path2.exists() {
                let dict = Dictionary::load_json_file(&path2, source)?;
                all_entries.extend(dict.entries);
                found = true;
                break;
            }
        }
        if !found {
            eprintln!(
                "Warning: dictionary source '{}' not found in any candidate path",
                source
            );
        }
    }

    if all_entries.is_empty() {
        anyhow::bail!("No dictionaries loaded");
    }

    Ok(Dictionary::from_entries(all_entries))
}

fn validate_dictionary(path: &Path) -> Result<()> {
    let entries: Vec<pctrl_core::DictionaryEntry> =
        serde_json::from_str(&fs::read_to_string(path)?)?;
    let mut errors = 0;
    for entry in &entries {
        if entry.word.is_empty() {
            eprintln!("Error: empty word");
            errors += 1;
        }
        if entry.pinyin.is_empty() {
            eprintln!("Error: empty pinyin for word '{}'", entry.word);
            errors += 1;
        }
        for py in &entry.pinyin {
            if !is_valid_pinyin(py) {
                eprintln!(
                    "Warning: suspicious pinyin '{}' for word '{}'",
                    py, entry.word
                );
            }
        }
    }
    if errors == 0 {
        println!("Dictionary is valid: {} entries", entries.len());
    } else {
        anyhow::bail!("Dictionary validation failed with {} errors", errors);
    }
    Ok(())
}

fn is_valid_pinyin(py: &str) -> bool {
    if py.len() < 2 {
        return false;
    }
    let Some(last) = py.chars().last() else {
        return false;
    };
    if !last.is_ascii_digit() || !("12345".contains(last)) {
        return false;
    }
    true
}
