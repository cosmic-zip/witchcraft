use std::path::Path;

use crate::core::consts::WITCH_SPELLS_ROOT_DIR;
use crate::core::core::*;
use crate::core::types::CommandRegistry;
use termimad::*;

pub fn read_markdown_from_path(argsv: &[String]) -> i32 {
    let command = search_value("page", argsv);
    let path = &format!("{}pages", WITCH_SPELLS_ROOT_DIR);
    let pages_paths = directory_lookup(Path::new(path));
    for page in pages_paths {
        if page.contains(&format!("/{}.md", command)) {
            let skin = MadSkin::default();
            skin.print_text(&std::fs::read_to_string(page).unwrap());
            return 0;
        }
    }

    return 2;
}

pub fn api() -> CommandRegistry {
    vec![("tldr", read_markdown_from_path)]
}
