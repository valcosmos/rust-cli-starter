use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            // v => anyhow::bail!("Unsupported format: {}", v),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutFormat::Json),
    //     "yaml" => Ok(OutFormat::Yaml),
    //     "toml" => Ok(OutFormat::Toml),
    //     _ => Err("Invalid format"),
    // }
}
