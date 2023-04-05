use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use itertools::Itertools;
use std::io;
use std::vec::Vec;

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

#[derive(Table)]
struct CodeUnit {
    #[table(title = "Display", justify = "Justify::Left")]
    display: String,
    #[table(title = "Code Unit")]
    code_unit: String,
    #[table(title = "Name")]
    name: String,
    #[table(title = "UTF-8 Byte(s)")]
    utf8_bytes: String,
    #[table(title = "Links")]
    urls: String,
}

fn char_to_bytestring(c: char) -> String {
    let mut buf = [0; 4];
    let bytes = c.encode_utf8(&mut buf).as_bytes();
    format!("{:08x}", bytes.iter().format(" "))
}

fn build_table(text: &String) -> Vec<CodeUnit> {
    let mut result = Vec::new();
    for val in text.chars() {
        result.push(CodeUnit {
            display: String::from(val),
            code_unit: String::from("tbd"),
            name: String::from("tbd"),
            utf8_bytes: char_to_bytestring(val),
            urls: String::from("tbd"),
        });
    }

    result
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

    let text = match cli.text {
        Some(text) => text,
        None => {
            // TODO: how to make it error on an empty string??
            // TODO: make it remove the line separator
            match stdin.read_line(&mut buffer) {
                Ok(_) => buffer,
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
    };

    let table = build_table(&text);
    print_stdout(table.with_title()).unwrap();

    println!("text analyzed: {}", text);
}
