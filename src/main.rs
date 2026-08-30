use std::error::Error;
use huffman::huffman::{HuffDecoder, HuffEncoder};

use clap::{Parser, CommandFactory};
use std::fs::File;
use std::io::{self, BufWriter, Read, IsTerminal, Write};
use std::time::Instant;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(help = "Reads from stdin if not provided")]
    input: Option<PathBuf>,

    #[arg(short, long)]
    decode: bool,
}

#[hotpath::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.input.is_none() && io::stdin().is_terminal() {
        Args::command().print_help().unwrap();
        std::process::exit(0);
    }

    // Decode
    if args.decode {
        let input = args.input.ok_or("Input file required for decoding")?;
        let (_decoder, decoded) = HuffDecoder::decode_file(input)?;
        println!("decoded:\n{}", String::from_utf8_lossy(&decoded));
        return Ok(());
    }

    // Encode
    let data = match args.input {
        Some(input) => std::fs::read(input)?,
        None => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
    };

    let encoder = HuffEncoder::from_vec(&data);
    let input_len = data.len();

    match args.output {
        Some(output) => {
            let file = File::create(output)?;
            let mut writer = BufWriter::new(file);
            encoder.write_header(&mut writer)?;
            let start = Instant::now();
            encoder.encode_all(&data, &mut writer)?;
            huffman::print_throughput("encoding throughput", input_len, start.elapsed());
        }
        None => {
            let mut buffer: Vec<u8> = Vec::new();
            encoder.write_header(&mut buffer)?;
            let start = Instant::now();
            encoder.encode_all(&data, &mut buffer)?;
            huffman::print_throughput("encoding throughput", input_len, start.elapsed());
            match buffer.len() {
                100.. => eprintln!("Refusing to print more than 100 bytes"),
                _ => println!("{encoder}\n{buffer:?}"),
            }
        }
    }

    Ok(())
}