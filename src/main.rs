use clap::{Parser, ValueEnum, CommandFactory};
use clap::error::ErrorKind;
use std::io;


// allowed input encodings
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputEncoding {
    /// UTF-8 directly pasted or typed from somewhere
    Utf8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// some help text
struct Cli {
    /// whether to show urls
    #[arg(short, long, default_value_t = false)]
    urls: bool,

    /// encoding to assume on input text
    #[arg(short, long, value_enum, default_value_t = InputEncoding::Utf8)]
    encoding: InputEncoding,

    /// Text to process
    #[arg(required = false)]
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("urls: {}", cli.urls);
    match cli.encoding {
        InputEncoding::Utf8 => {
            println!("encoding: utf8");
        }
    }

    match cli.text {
        Some(text) => {
            println!("text: {}", text);
        },
        None => {
            // TODO: how to make it error on an empty string?? 
            match stdin.read_line(&mut buffer) {
                Ok(text) => {
                    println!("text (stdin): {}", buffer);
                },
                Err(_) => {
                    let mut cmd = Cli::command();
                    cmd.error(
                        ErrorKind::MissingRequiredArgument,
                        "Must provide arguments or standard input",
                    )
                    .exit();
                }
            }
        }
    }
}
