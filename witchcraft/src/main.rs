use core::{consts::*, core::*};
use modules::{
    binds::binds,
    blackcat::blackcat,
    network::network,
    osint::osint,
    seth::seth,
    tldr::tldr,
};
use std::io::{self, Write};
use std::process;

mod core;
mod modules;

fn init(argsv: Vec<String>) {
    let arg_name = argsv[1].as_str();

    let mut command_registry = Vec::new();
    command_registry.extend(binds::api());
    command_registry.extend(blackcat::api());
    command_registry.extend(network::api());
    command_registry.extend(osint::api());
    command_registry.extend(seth::api());
    command_registry.extend(tldr::api());

    if arg_name == "help" || arg_name == "h" {
        magic_docs();
    }

    if arg_name == "manual" || arg_name == "m" {
        raise(MAN_HEADER, "");
        magic_docs();
    }

    if arg_name == "version" || arg_name == "v" {
        show_version();
    }

    let code = command_shell(command_registry, &argsv);
    if code == 8080 {
        let exit = flawless_entry_point(&argsv);
        process::exit(exit);
    }
    process::exit(code);
}

fn main() {
    let argsv = readargs();
    if argsv.len() % 2 != 0 {
        println!("{}", PANZER_MAID);
        println!("{}", BOTTOM_TEXT);
        io::stdout().flush().unwrap();
        process::exit(1);
    }

    init(argsv);
}

#[cfg(test)]
mod test;
