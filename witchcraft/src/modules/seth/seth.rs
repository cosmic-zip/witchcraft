use crate::core::types::CommandRegistry;
use crate::modules::seth::base_sixfour::base_sixfour;
// use crate::modules::seth::catfish::*;
use crate::modules::seth::{qrcode::*, server::*};

pub fn api() -> CommandRegistry {
    vec![
        ("qrcode", gen_qrcode_from_argsv),
        ("base64", base_sixfour),
        ("server.eviltwin", evil_server),
    ]
}
