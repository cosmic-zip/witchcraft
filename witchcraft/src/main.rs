use colored::*;
use core::{consts::*, core::*};
use modules::{
    binds::binds::{self, flawless_entry_point},
    blackcat::blackcat,
    network::network,
    osint::osint,
    seth::seth,
    tldr::tldr,
};
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

mod core;
mod modules;

fn init(argsv: Vec<String>) {
    let arg_name = argsv[1].as_str();

    let mut join_closures = Vec::new();
    join_closures.extend(binds::api());
    join_closures.extend(blackcat::api());
    join_closures.extend(network::api());
    join_closures.extend(osint::api());
    join_closures.extend(seth::api());
    join_closures.extend(tldr::api());

    if arg_name == "help" || arg_name == "h" {
        magic_docs();
    }

    if arg_name == "manual" || arg_name == "h" {
        raise(MAN_HEADER, "");
        magic_docs();
    }

    if arg_name == "version" || arg_name == "v" {
        show_version();
    }

    let code = closure_shell(join_closures, &argsv);
    if code == 11223300 {
        let code = flawless_entry_point(&argsv);
        process::exit(code);
    }

    process::exit(code);
}

fn main() {
    let argsv = readargs();
    if argsv.len() % 2 != 0 {
        println!("{}", PANZER_MAID);
        println!("{}", BOTTOM_TEXT);
        io::stdout().flush().unwrap();
        process::exit(42);
    }

    // Shared flag to signal completion
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = Arc::clone(&done);

    // Spawn the timer thread
    let timer = thread::spawn(move || {
        let mut counter = 0;
        let mut icon = 0;
        let spinner_chars = vec!['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'];

        loop {
            thread::sleep(Duration::from_millis(100));
            counter += 100;

            let msg = format!(
                "[] Processing, please wait (or not, do whatever you want) : {} milliseconds {}\r",
                counter, spinner_chars[icon]
            );
            print!("{}", msg.bold().magenta());
            io::stdout().flush().unwrap();

            icon = (icon + 1) % spinner_chars.len();

            if done.load(Ordering::SeqCst) || counter / 1000 == 43200 {
                if counter == 43200 {
                    println!("\nProcessing took too long: 12-hour limit reached.");
                }
                break;
            }
        }
    });

    let init = thread::spawn(move || {
        init(argsv);
        done_clone.store(true, Ordering::Relaxed);
    });

    init.join().unwrap();
    timer.join().unwrap();
}

#[cfg(test)]
mod test;
