// rcli csv -i input.csv -o output.json --header -d ','

use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};
// use zxcvbn::zxcvbn;

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
        SubCommand::GenPass(opts) => {
            // println!("Generate password: {:?}", opts)
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("{:?}", opts);
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts);
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }

    Ok(())
    // println!("{:?}", opts);
}
