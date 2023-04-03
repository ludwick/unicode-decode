# unicode-decode

Command-line tool to decode &amp; explain unicode strings. This is just here
to learn a bit about Rust and how it manipulates Unicode strings. Currently
limited to UTF-8 input.

## Usage

Pass in a string as an argument:

```
unicode-decode "some interesting string"
```

Or from standard input:

```
echo "some interesting string" | unicode-decode
```

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
* `-u` / `--urls`: include urls in output
* `-e` / `--encoding`: the type of data (only allowed: utf8)
* `-h` / `--help`: print help message

## Features

* Shows each byte and what it means
* May output URLs to emojipedia and other sources

## License

MIT
