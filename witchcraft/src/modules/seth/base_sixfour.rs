use base64::engine::*;

use crate::core::core::{raise, search_value};

pub fn base_sixfour(argsv: &[String]) -> i32 {
    let option = search_value("option", argsv);
    let data = search_value("data", argsv);

    if option == "encode" {
        let out = general_purpose::STANDARD.encode(data.as_bytes());
        raise(&out, "message");
    }

    if option == "decode" {
        let out = general_purpose::STANDARD.decode(data).unwrap();
        let decoded_str = String::from_utf8(out).expect("Invalid UTF-8");
        raise(&decoded_str, "message");
    }

    0
}
