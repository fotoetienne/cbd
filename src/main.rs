use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use clap::Parser;
use ciborium::from_reader;
use ciborium::into_writer;
use base64::engine::general_purpose;
use base64::Engine;

/**
 * # Compact Binary Decoder #
 *
 * Library for decoding and encoding compact binary (cbor) data
 *
 * ## Usage ##
 *
 * Decode CBOR from stdin and output JSON:
 * ```shell
 * $ cat file.cbor | cbd
 * {"key": "value"}
 * ```
 *
 * Encode JSON from stdin and output CBOR:
 * ```shell
 * $ cat file.json | cbd -e
 * ?ckeyevalue%
 * ```
 *
 * ## Installation ##
 * ```shell
 * $cargo install cbd
 * ```
 */

fn main() {
    let cli = Cli::parse();
    if cli.encode {
        cbor_encode(cli.base64);
    } else {
        cbor_decode();
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    encode: bool,

    #[clap(short, long)]
    base64: bool,
}

fn try_base64_decode(input: &Vec<u8>) -> Result<Vec<u8>, CbdError> {
    let text = std::str::from_utf8(input).map_err(|e| CbdError {
        message: "Failed to decode input as utf8".to_string(),
        source: Some(Box::new(e)),
    })?.trim_end();
    if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(text) {
        return Ok(bytes)
    }
    if let Ok(bytes) = general_purpose::STANDARD.decode(text) {
        return Ok(bytes)
    }
    if let Ok(bytes) = general_purpose::URL_SAFE.decode(text) {
        return Ok(bytes)
    }
    if let Ok(bytes) = general_purpose::STANDARD_NO_PAD.decode(text) {
        return Ok(bytes)
    }
    Err(CbdError {
        message: "Failed to decode base64".to_string(),
        source: None,
    })
}

#[derive(Debug)]
struct CbdError {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Display for CbdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CbdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

fn base64_encode(input: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(input)
}

fn cbor_decode() {
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin");
    let json = decode(&buffer).expect("Failed to decode CBOR");
    println!("{}", json);
}

/**
 * Decode input to JSON
 * Try base64 encoded cbor first, then raw cbor
 */
fn decode(input: &Vec<u8>) -> Result<String, CbdError> {
    if let Ok(cbor) = try_base64_decode(input) {
        try_cbor2json(&cbor)
    } else {
        try_cbor2json(input)
    }
}

fn try_cbor2json(cbor: &Vec<u8>) -> Result<String, CbdError> {
    let value: ciborium::Value = from_reader(&cbor[..]).map_err(|e| CbdError {
        message: "Failed to decode CBOR".to_string(),
        source: Some(Box::new(e)),
    })?;
    serde_json::to_string(&value).map_err(|e| CbdError {
        message: "Failed to encode JSON".to_string(),
        source: Some(Box::new(e)),
    })
}

fn cbor_encode(base64: bool) {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    let json = buffer.trim();
    let cbor = json2cbor(json);
    if base64 {
        let base64 = base64_encode(&cbor);
        std::io::stdout().write_all(base64.as_bytes()).expect("Failed to write to stdout");
    } else {
        std::io::stdout().write_all(&cbor).expect("Failed to write to stdout");
    }
}

fn json2cbor(json: &str) -> Vec<u8> {
    let value: serde_json::Value = serde_json::from_str(json).expect("Failed to decode JSON");
    let mut writer = Vec::new();
    into_writer(&value, &mut writer).expect("Failed to encode CBOR");
    writer
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON_IN: &str = r#"[{"key1":"value1","key2":"value2"},{"foo":"bar"},true,false,0,1.0]"#;

    #[test]
    fn test_cbor2json() {
        let cbor = vec![161, 97, 107, 97, 118];
        let json = try_cbor2json(&cbor).unwrap();
        assert_eq!(json, r#"{"k":"v"}"#);
    }

    #[test]
    fn test_json2cbor() {
        let json = r#"{"k":"v"}"#;
        let cbor = json2cbor(json);
        assert_eq!(cbor, vec![161, 97, 107, 97, 118]);
    }

    #[test]
    fn test_decode_cbor() {
        let cbor = json2cbor(JSON_IN);
        let json = decode(&cbor).unwrap();
        assert_eq!(json, JSON_IN);
    }

    #[test]
    fn test_decode_base64_standard() {
        let base64 = general_purpose::STANDARD.encode(json2cbor(JSON_IN));
        let bytes = dbg!(base64).as_bytes().to_vec();
        let json_out = decode(&bytes).unwrap();
        assert_eq!(JSON_IN, json_out);
    }

    #[test]
    fn test_decode_base64_standard_no_pad() {
        let base64 = general_purpose::STANDARD_NO_PAD.encode(json2cbor(JSON_IN));
        let bytes = dbg!(base64).as_bytes().to_vec();
        let json_out = decode(&bytes).unwrap();
        assert_eq!(JSON_IN, json_out);
    }

    #[test]
    fn test_decode_base64_url_safe() {
        let base64 = general_purpose::URL_SAFE.encode(json2cbor(JSON_IN));
        let bytes = dbg!(base64).as_bytes().to_vec();
        let json_out = decode(&bytes).unwrap();
        assert_eq!(JSON_IN, json_out);
    }

    #[test]
    fn test_decode_base64_url_safe_no_pad() {
        let base64 = general_purpose::URL_SAFE_NO_PAD.encode(json2cbor(JSON_IN));
        let bytes = dbg!(base64).as_bytes().to_vec();
        let json_out = decode(&bytes).unwrap();
        assert_eq!(JSON_IN, json_out);
    }
}
