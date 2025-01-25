# Compact Binary Decoder #

Library for decoding and encoding compact binary (cbor) data

## Usage ##

Decode CBOR from stdin and output JSON:
```shell
$ cat file.cbor | cbd
{"key": "value"}
```
*
Encode JSON from stdin and output CBOR:
```shell
$ cat file.json | cbd -e
?ckeyevalue%
```
*
## Installation ##
```shell
$cargo install cbd
```
