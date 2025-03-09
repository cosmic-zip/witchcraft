use base64::engine::*;

use crate::core::core::{raise, search_value};

pub fn base_sixfour(argsv: &[String]) -> i32 {
    let option = search_value("option", argsv);
    let data = search_value("data", argsv);

    match option.as_str() {
        "encode" => {
            let out = general_purpose::STANDARD.encode(data.as_bytes());
            raise(&out, "message");
        },
        "decode" => {
            // Use improved error handling from previous comment
            match general_purpose::STANDARD.decode(data) {
                Ok(out) => {
                    match String::from_utf8(out) {
                        Ok(decoded_str) => raise(&decoded_str, "message"),
                        Err(_) => {
                            raise("Decoded data is not valid UTF-8", "error");
                            return 1;
                        }
                    }
                },
                Err(e) => {
                    raise(&format!("Failed to decode Base64 data: {}", e), "error");
                    return 1;
                }
            }
        },
        _ => {
            raise(&format!("Invalid option: '{}'. Use 'encode' or 'decode'", option), "error");
            return 1;
        }
    }

    0
}
