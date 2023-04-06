# unicode-decode

Command-line tool to decode &amp; explain unicode strings. This repository is
not meant to be anything anyone should depend on, but is instead a useful to
me tool I decided to write as a means of refreshing my knowledge of Rust (previous
work was under Edition 2018!) and also to understand Unicode better.
This is currently limited to analyzing UTF-8 input.

## Install

Right now, clone the repo. Then run `cargo run --  --help` (assuming you have cargo
etc installed).

## Usage

Pass in a string as an argument:

```
unicode-decode "some interesting string"
```

Or from standard input:

```
echo "some interesting string" | unicode-decode
```

If you don't pass in input by one of these mechanisms, the tool will wait for
input followed by a newline:

```
unicode-decode<enter>
some interesting string
```

To add URLs to useful websitese for a given string:

```
echo "what" | unicode-decode -u -
```

Options:

* Positional arguments: string to process
* `-e` / `--encoding`: the type of data (only allowed: utf8)
* `-h` / `--help`: print help message

## Features

* Shows each Unicode code point and its name
* Shows the byte(s) it is represented in utf-8

## License

MIT
