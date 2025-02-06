# Concise Binary Decoder #

Library for decoding and encoding Concise Binary ([cbor](https://cbor.io)) Data

## Installation ##
```shell
$ cargo install cbd
```

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

**Decode CBOR into JSON:**
```shell
$ cat file.cbor | cbd
{"key": "value"}
```

**Decode base64 encoded CBOR into JSON:**
```shell
$ echo 'oWNrZXlldmFsdWU' | cbd
{"key": "value"}
```

**Encode JSON into CBOR:**
```shell
$ cat file.json | cbd -e
?ckeyevalue
```

**Encode JSON into base64 encoded CBOR:**
```shell
$ cat file.json | cbd -e --base64
oWNrZXlldmFsdWU
```
