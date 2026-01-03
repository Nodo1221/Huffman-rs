use std::error::Error;
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
    
    if args.input.is_none() && io::stdin().is_terminal() {
        Args::command().print_help().unwrap();
        std::process::exit(0);
    }

    if args.decode {
        let input = args.input.ok_or("Input file required for decoding")?;
        let (_decoder, decoded) = HuffDecoder::decode_file(input)?;

        match args.output {
            Some(output) => println!("decoded:\n{}", String::from_utf8_lossy(&decoded)),
            _ => println!("decoded:\n{}", String::from_utf8_lossy(&decoded)),
        }

        return Ok(());
    }

    let (encoder, encoded) = match args.input {
        Some(input) => HuffEncoder::encode_file(input)?,
        None => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            let encoder = HuffEncoder::from_vec(&buffer);
            let encoded = encoder.encode(&buffer);

            (encoder, encoded)
        }
    };

    match args.output {
        Some(output) => encoder.write_file(output, &encoded)?,
        None => match encoded.data.len() {
            100.. => eprintln!("Refusing to print more than 100 bytes"),
            _ => println!("{}", encoded)
        }
    }

    Ok(())
}