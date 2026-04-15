use pctrl_core::PronunciationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Json,
    Segmented,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Self::Plain),
            "json" => Ok(Self::Json),
            "segmented" => Ok(Self::Segmented),
            _ => anyhow::bail!("unsupported format: {}", s),
        }
    }
}

pub fn format_result(result: &PronunciationResult, format: OutputFormat) -> anyhow::Result<String> {
    match format {
        OutputFormat::Plain => Ok(result.to_plain()),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(result)?),
        OutputFormat::Segmented => Ok(result.to_segmented()),
    }
}
