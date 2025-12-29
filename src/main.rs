use std::error::Error;
use std::str::EncodeUtf16;
use huffman::huffman::{HuffDecoder, HuffEncoder};

use clap::{Parser, CommandFactory};
use std::path::PathBuf;
use std::io::{self, Read, IsTerminal};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(help = "Reads from stdin if not provided")]
    input: Option<PathBuf>,

    #[arg(short, long)]
    decode: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    if !args.decode {
        // Read input from file or stdin
        let (encoder, encoded) = match args.input {
            Some(input) => HuffEncoder::encode_file(input)?,
            None => {
                if io::stdin().is_terminal() {
                    Args::command().print_help().unwrap();
                    // Args::parse_from(&["--help"]);
                    std::process::exit(0);
                }
                
                let mut buffer = Vec::new();
                io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin");

                let encoder = HuffEncoder::from_vec(&buffer);
                let encoded = encoder.encode(&buffer);

                (encoder, encoded)
            }
        };

        if let Some(output) = args.output {
            encoder.write_file(output, &encoded)?;
        }

        else {
            println!("{}", encoded);
        }
    }

    else {
        if let Some(input) = args.input {
            let (decoder, decoded) = HuffDecoder::decode_file(input)?;
            println!("decoded:\n{}", String::from_utf8_lossy(&decoded));
        }
    }

    Ok(())
}