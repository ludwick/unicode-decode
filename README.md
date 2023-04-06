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

Example:

```
☰ ~ echo 👮🏽‍♀️ | ./target/debug/unicode-decode
+---------+------------+-----------------------------------+---------------+
| Display | Code Point | Name                              | UTF-8 Byte(s) |
+---------+------------+-----------------------------------+---------------+
| 👮      |      1f46e | POLICE OFFICER                    |   f0 9f 91 ae |
+---------+------------+-----------------------------------+---------------+
| 🏽      |      1f3fd | EMOJI MODIFIER FITZPATRICK TYPE-4 |   f0 9f 8f bd |
+---------+------------+-----------------------------------+---------------+
|         |       200d | ZERO WIDTH JOINER                 |      e2 80 8d |
+---------+------------+-----------------------------------+---------------+
| ♀       |       2640 | FEMALE SIGN                       |      e2 99 80 |
+---------+------------+-----------------------------------+---------------+
| ️        |       fe0f | VARIATION SELECTOR-16             |      ef b8 8f |
+---------+------------+-----------------------------------+---------------+
|         |       000a | LINE FEED (LF)                    |             a |
+---------+------------+-----------------------------------+---------------+
text analyzed: 👮🏽‍♀️

```

Pass in a string as an argument:

```
unicode-decode "some interesting string"
```

If you don't pass in input by one of these mechanisms, the tool will wait for
input followed by a newline:

```
unicode-decode<enter>
some interesting string
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
