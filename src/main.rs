use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use itertools::Itertools;
use std::io;
use std::vec::Vec;
use std::collections::HashMap;

pub mod names;
pub mod control;


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
    #[table(title = "Code Point", justify = "Justify::Right")]
    code_point: String,
    #[table(title = "Name", justify = "Justify::Left")]
    name: String,
    #[table(title = "UTF-8 Byte(s)", justify = "Justify::Right")]
    utf8_bytes: String,
}

struct NameDatabase {
    db: HashMap<u32, &'static str>
}

impl NameDatabase {
    pub fn new() -> NameDatabase {
        let iter = names::NAMES.iter().map(|(name, code)| (*code, *name)).chain(control::CONTROL.iter().copied());
        // NameDatabase { db: iter.collect()HashMap::from_iter(iter) }
        NameDatabase { db: iter.collect() }
    }

    pub fn for_char(&self, c: char) -> Option<&str> {
        self.db.get(&(c as u32)).copied()
    }
}


fn char_to_bytestring(c: char) -> String {
    let mut buf = [0; 4];
    let bytes = c.encode_utf8(&mut buf).as_bytes();
    format!("{:x}", bytes.iter().format(" "))
}

fn build_table(text: &String) -> Vec<CodeUnit> {
    let names = NameDatabase::new();
    let mut result = Vec::new();
    for val in text.chars() {
        result.push(CodeUnit {
            display: String::from(val),
            code_point: format!("{:04x}", val as u32),
            name: String::from(match names.for_char(val) { Some(v) => v, None => "unknown" }),
            utf8_bytes: char_to_bytestring(val),
        });
    }

    result
}

fn main() {
    let cli = Cli::parse();
    let mut buffer = String::new();
    let stdin = io::stdin();

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
