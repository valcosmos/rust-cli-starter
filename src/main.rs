// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // "output.json".into()
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        } // let mut reader = Reader::from_path(opts.input)?;
          // let mut ret = Vec::with_capacity(128);
          // // let records = reader
          // //     .deserialize()
          // //     .map(|record| record.unwrap())
          // //     .collect::<Vec<Player>>();
          // // println!("{:?}", records);
          // for result in reader.deserialize() {
          //     let record: Player = result?;
          //     println!("{:?}", record);
          //     ret.push(record);
          // }
          // let json  = serde_json::to_string_pretty(&ret)?;
          // fs::write(opts.output, json)?;
    }

    Ok(())
    // println!("{:?}", opts);
}
