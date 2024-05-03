use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter()  使用headers的迭代器
        // record.iter()  使用record的迭代器
        // zip()  将两个迭代器合并为一个远足的迭代器 [(header, record), ...]
        // collect::<Value>()  将元组的迭代器转换为 JSON value

        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        // println!("{:?}", record);
        ret.push(json_value);
    }
    // let json = serde_json::to_string_pretty(&ret)?;
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        //    OutputFormat::Toml=>toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
