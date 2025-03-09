use crate::{
    core::types::CommandRegistry,
    modules::blackcat::backend::{blackcat_av, blackcat_chkrootkit_bind},
};

pub fn api() -> CommandRegistry {
    vec![
        ("blackcat.av", blackcat_av),
        ("blackcat.rootkit", blackcat_chkrootkit_bind),
    ]
}
