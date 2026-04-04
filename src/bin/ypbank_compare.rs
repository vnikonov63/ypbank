use clap::Parser;
use ypbank::{read_storage, Format};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: Format,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    format2: Format,
}

fn main() {
    let args = Args::parse();

    let storage1 = read_storage(&args.file1, args.format1).unwrap();
    let storage2 = read_storage(&args.file2, args.format2).unwrap();

    print!("# Output: The transaction records in '{}' and '{}' are ", &args.file1, &args.file2);
    if storage1 == storage2 {
        println!("identical");
    } else {
        println!("not identical");
    }
}
