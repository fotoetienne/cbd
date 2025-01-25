# Concise Binary Decoder #

Library for decoding and encoding Concise Binary ([cbor](https://cbor.io)) Data

## Usage ##

**Help:**
```shell
$ cbd --help
Usage: cbd [OPTIONS]

Options:
  -e, --encode  
  -b, --base64  
  -h, --help    Print help
```

**Decode CBOR from stdin and output JSON:**
```shell
$ cat file.cbor | cbd
{"key": "value"}
```

**Encode JSON from stdin and output CBOR:**
```shell
$ cat file.json | cbd -e
?ckeyevalue%
```

## Installation ##
```shell
$ cargo install cbd
```
