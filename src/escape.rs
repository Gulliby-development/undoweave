//! Byte-oriented insertion escape decoding and output rendering.

use crate::Failure;

/// Decode the supported insertion escapes, rejecting malformed escape sequences.
pub fn unescape(input: &[u8]) -> Result<Vec<u8>, Failure> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        if input[index] != b'\\' {
            output.push(input[index]);
            index += 1;
            continue;
        }
        index += 1;
        let Some(&escape) = input.get(index) else {
            return Err(Failure::new(1, "bad escape"));
        };
        match escape {
            b'n' => {
                output.push(b'\n');
                index += 1;
            }
            b't' => {
                output.push(b'\t');
                index += 1;
            }
            b'\\' => {
                output.push(b'\\');
                index += 1;
            }
            b'x' => {
                let (Some(&high), Some(&low)) = (input.get(index + 1), input.get(index + 2)) else {
                    return Err(Failure::new(1, "bad escape"));
                };
                let (Some(high), Some(low)) = (hex(high), hex(low)) else {
                    return Err(Failure::new(1, "bad escape"));
                };
                output.push(high * 16 + low);
                index += 3;
            }
            _ => return Err(Failure::new(1, "bad escape")),
        }
    }
    Ok(output)
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Render bytes as the deterministic ASCII representation used in reports.
pub fn shown(input: &[u8]) -> String {
    let mut output = String::with_capacity(input.len());
    for &byte in input {
        match byte {
            b'\\' => output.push_str("\\\\"),
            b'\n' => output.push_str("\\n"),
            b'\t' => output.push_str("\\t"),
            0x20..=0x7e => output.push(char::from(byte)),
            _ => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                output.push_str("\\x");
                output.push(char::from(HEX[usize::from(byte >> 4)]));
                output.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }
    output
}
