use crate::core::types::CommandRegistry;
use crate::modules::network::ddos::*;
use crate::modules::network::map_network::*;

pub fn api() -> CommandRegistry {
    vec![
        ("dos.longpw", dos_long_auth_span),
        ("dos.spam", dos_simple_get_span),
        ("map.dns", map_dns),
    ]
}
