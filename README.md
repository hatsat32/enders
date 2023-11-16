# Ende

Simple encode/decode tool written in rust!

## Usage

```
$ ende
Simple encode/decode tool written in rust!

Usage: ende <COMMAND>

Commands:
  enb64    base64 encode given text
  deb64    base64 encode given text
  enb32    base32 encode given text
  deb32    base32 encode given text
  enb16    base16 encode given text
  deb16    base16 encode given text
  enhex    hex encode given text
  dehex    hex encode given text
  enhtml   HTML encode given text
  dehtml   HTML encode given text
  enurl    URL encode given text
  deurl    URL decode given text
  enrot13  Rot13 encode given text
  derot13  Rot13 encode given text
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

URL encode

```
$ ende enurl hello world
hello%20world

$ ende deurl hello%20world
hello world
```

Base16

```
$ ende enb16 hello world
68656C6C6F20776F726C64

$ ende deb16 68656C6C6F20776F726C64
hello world
```

## TODO

Encodings:

- [x] base64
- [x] base32
- [x] base16
- [x] url
- [x] html
- [x] hex

Converts:

- [ ] csv2json
- [ ] json2csv

Encryption:

- [x] rot13
