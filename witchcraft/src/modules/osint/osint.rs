use crate::core::types::CommandRegistry;
use crate::modules::osint::lookup::*;
use crate::modules::osint::meta_search::social_links;

pub fn api() -> CommandRegistry {
    vec![
        ("search.ans", search_ans),
        ("search.geoloc", search_geoloc),
        ("search.proxy", search_proxy),
        ("search.ipscore", cinsscore),
        ("search.meta", social_links),
    ]
}
