use clap::{Parser, ValueEnum};


// allowed input encodings
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputEncoding {
    Utf8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// text to process
    #[arg(short, long)]
    text: String,

    /// whether to show urls
    #[arg(short, long, default_value_t = false)]
    urls: bool,

    /// encoding to assume
    #[arg(short, long, value_enum)]
    encoding: InputEncoding,
}

fn main() {
    let args = Args::parse();

    println!("text: {}", args.text);
    println!("urls: {}", args.urls);
    match args.encoding {
        InputEncoding::Utf8 => {
            println!("encoding: utf8");
        }
    }
}
