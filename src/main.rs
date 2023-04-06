use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use itertools::Itertools;
use std::io;
use std::vec::Vec;
use std::collections::HashMap;

pub mod names;


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
    #[table(title = "Code Unit", justify = "Justify::Right")]
    code_unit: String,
    #[table(title = "Name", justify = "Justify::Left")]
    name: String,
    #[table(title = "UTF-8 Byte(s)", justify = "Justify::Right")]
    utf8_bytes: String,
    #[table(title = "Links")]
    urls: String,
}

struct NameDatabase {
    db: HashMap<u32, &'static str>
}

impl NameDatabase {
    pub fn new() -> NameDatabase {
        let iter = names::NAMES.into_iter().map(|(name, code)| (*code, *name));
        NameDatabase { db: HashMap::from_iter(iter) }
    }

    pub fn for_char(&self, c: char) -> Option<&str> {
        self.db.get(&(c as u32)).copied()
    }
}


fn char_to_bytestring(c: char) -> String {
    let mut buf = [0; 4];
    let bytes = c.encode_utf8(&mut buf).as_bytes();
    format!("{:08x}", bytes.iter().format(" "))
}

fn build_table(text: &String) -> Vec<CodeUnit> {
    let names = NameDatabase::new();
    let mut result = Vec::new();
    for val in text.chars() {
        result.push(CodeUnit {
            display: String::from(val),
            code_unit: format!("{:04x}", val as u32),
            name: String::from(match names.for_char(val) { Some(v) => v, None => "unknown" }),
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
            // TODO: make it remove the line separator if present (unclear
            // but when interactively waiting, the newline seems to be removed
            // so maybe it's just an EOF in that case?)
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
