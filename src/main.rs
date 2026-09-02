use std::error::Error;
use huffman::huffman::{HuffDecoder, HuffEncoder};
use huffman::bits::BitData;

use clap::{Parser, CommandFactory};
use std::fs::File;
use std::io::{self, Read, Write, BufWriter, IsTerminal};
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

    if args.decode {
        let input = args.input.ok_or("Input file required for decoding")?;
        let (_decoder, decoded) = HuffDecoder::decode_file(input)?;
        println!("decoded:\n{}", String::from_utf8_lossy(&decoded));
        return Ok(());
    }

    let start = Instant::now();

    let mut in_file: Box<dyn Read> = match args.input {
        Some(ref path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };

    let mut buf = Vec::new();
    in_file.read_to_end(&mut buf)?;
    let input_len = buf.len();

    let encoder = HuffEncoder::from_data(&buf);

    match args.output {
        Some(path) => {
            let mut out_file = BufWriter::with_capacity(1024 * 8, File::create(path)?);
            encoder.write_header(&mut out_file)?;
            encoder.encode_all(&mut buf.as_slice(), &mut out_file)?;
            out_file.flush()?;
        }
        None => {
            println!("{}", encoder);
            let mut out_buf = BitData::new();
            encoder.encode_chunk(&buf, &mut out_buf);
            println!("{}", out_buf);
        }
    }

    huffman::print_throughput("total throughput", input_len, start.elapsed());

    Ok(())
}