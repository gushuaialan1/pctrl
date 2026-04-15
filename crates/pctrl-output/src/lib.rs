use pctrl_core::PronunciationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Json,
    Segmented,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
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
