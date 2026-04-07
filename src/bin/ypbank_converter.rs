use clap::Parser;
use ypbank::{read_storage, Format};
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long)]
    input_format: Format,

    #[arg(long)]
    output_format: Format,
}

fn main() {
    let args = Args::parse();
    let storage = read_storage(&args.input, args.input_format).unwrap();

    let mut stdout = io::stdout();



    match args.output_format {
        Format::Binary => {
            storage.to_bin(&mut stdout);
        }
        Format::Csv => {
            storage.to_csv(&mut stdout);
        }
        Format::Txt => {
            storage.to_txt(&mut stdout);
        }
    }
}
