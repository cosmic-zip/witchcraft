use crate::core::types::CommandRegistry;
// use crate::modules::seth::catfish::*;
use crate::modules::seth::{qrcode::*, server::*};

pub fn api() -> CommandRegistry {
    vec![
        ("qrcode", gen_qrcode_from_argsv),
        ("server.eviltwin", evil_server),
    ]
}
