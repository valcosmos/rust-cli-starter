// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
            // let mut reader = Reader::from_path(opts.input)?;
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

