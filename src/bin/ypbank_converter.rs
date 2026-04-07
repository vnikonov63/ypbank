use clap::Parser;
use ypbank::{read_storage, Format};
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: Format,

    #[arg(long)]
    format2: Format,
}

fn main() {
    let args = Args::parse();
    let storage = read_storage(&args.file1, args.format1).unwrap();

    let mut stdout = io::stdout();



    match args.format2 {
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
