use clap::Parser;
use std::io;
use ypbank::{Format, read_storage};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long)]
    input_format: Format,

    #[arg(long)]
    output_format: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let storage = read_storage(&args.input, args.input_format)?;

    let mut stdout = io::stdout();

    match args.output_format {
        Format::Binary => {
            let _ = storage.to_bin(&mut stdout);
        }
        Format::Csv => {
            let _ = storage.to_csv(&mut stdout);
        }
        Format::Txt => {
            let _ = storage.to_txt(&mut stdout);
        }
    }

    Ok(())
}
