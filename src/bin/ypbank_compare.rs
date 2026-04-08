use clap::Parser;
use ypbank::{Format, read_storage};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let storage1 = read_storage(&args.file1, args.format1)?;
    let storage2 = read_storage(&args.file2, args.format2)?;

    print!(
        "# Output: The transaction records in '{}' and '{}' are ",
        &args.file1, &args.file2
    );
    if storage1 == storage2 {
        println!("identical");
    } else {
        println!("not identical");
    }

    Ok(())
}
