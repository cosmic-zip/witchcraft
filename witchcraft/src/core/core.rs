use super::types::CommandRegistry;
use super::witchrc::witchy_readrc_value;
use crate::core::consts::*;
use crate::core::data::*;
use crate::core::logger::core_logger;
use crate::core::structs::DataSet;
use chrono;
use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::net::IpAddr;
use std::path::Path;
use std::process::{Command, Output};
use std::str::FromStr;
use termimad::crossterm::style::Stylize;

/// Reads command line arguments and returns them as a `Vec<String>`.
///
/// # Returns
/// A `Vec<String>` containing the command line arguments passed to the program.
///
/// # Example
/// ```
/// let args = readargs();
/// println!("{:?}", args);  // Example: ["program_name", "arg1", "arg2"]
/// ```
pub fn readargs() -> Vec<String> {
    env::args().collect()
}

/// Returns the full path to the `spellbook` folder by appending the given `path` to the
/// root directory path defined by `WITCH_SPELLS_ROOT_DIR`.
///
/// # Arguments
///
/// * `path` - A string slice representing the relative path to a specific file or folder within
///   the `spellbook` directory.
///
/// # Returns
///
/// A `String` containing the complete path, combining the root directory path and the provided relative `path`.
///
/// # Example
///
/// ```rust
/// let spell_path = get_spellbook_path("ancient_spells.txt");
/// assert_eq!(spell_path, "/var/witchcraft/spellbook/ancient_spells.txt");
/// ```
///
/// Note: Ensure `WITCH_SPELLS_ROOT_DIR` is set to the root path (e.g., "/var/witchcraft/spellbook/")
/// for the returned path to be accurate.
pub fn get_spellbook_path(path: &str) -> String {
    let if_root = get_os_env("WITCH_SPELLS_ROOT_DIR");
    // raise(&if_root, "good"); Show WITCH_SPELLS_ROOT_DIR value if they exists
    if if_root.is_empty() {
        return format!("{}{}", WITCH_SPELLS_ROOT_DIR, path);
    }
    format!("{}{}", if_root, path)
}

/// Returns the current date and time as a `String`.
///
/// # Returns
/// A `String` representing the current date and time in a human-readable format.
///
/// # Example
/// ```
/// let current_datetime = datetime_now();
/// println!("{}", current_datetime);  // Example: "2024-10-23 14:30:45"
/// ```
pub fn datetime_now() -> String {
    let time = chrono::offset::Local::now();
    return time.to_string();
}

/// Raises a formatted message based on the provided warning type.
///
/// # Arguments
///
/// * `arg` - A string slice that holds the message to be displayed.
/// * `warning_type` - A string slice that specifies the type of warning.
///   Valid options are:
///   - 🟣 [ system says ] :: "message": Indicates a general message.
///   - 🟢 [ well done ] :: "done": Indicates a successful operation.
///   - 🔴 [ fail ] :: "fail": Indicates a failure.
///   - 🟠 [ warning ] :: "warning": Indicates a warning condition.
///   - 💀 [ bruh ] :: "error": Indicates an error.
///   - 🔘 [ entry point ] :: "entry": Indicates an entry point message.
///   - 💖 [ GOOD ] :: "good": Indicates an lovely message.
///
/// # Returns
///
/// A formatted string that combines the selected warning type with the provided message.
/// If an invalid `warning_type` is given, an error message will be returned.
///
/// # Example
///
/// ```rust
/// let result = raise("Operation completed", "done");
/// println!("{}", result); // Output: "🟢 [ DONE ] :: Operation completed"
/// ```
pub fn raise(arg: &str, warning_type: &str) -> String {
    let opts = [
        "🟣 [ system says ] ::",
        "🟢 [ well done ] ::",
        "🔴 [ fail ] ::",
        "🟠 [ warning ] ::",
        "💀 [ bruh ] ::",
        "🔘 [ entry point ] ::",
        "💖 [ GOOD ] ::",
    ];

    // Match the warning_type to find the corresponding option
    let out = match warning_type {
        "message" => opts[0],
        "done" => opts[1],
        "fail" => opts[2],
        "warning" => opts[3],
        "error" => opts[4],
        "entry" => opts[5],
        "good" => opts[6],
        _ => "", // Empty message
    };

    let mut formatted_output = format!("{} {}", out.to_uppercase(), arg);
    if formatted_output.len() < 120 {
        formatted_output = format!(
            "{}{}",
            formatted_output,
            " ".repeat(120 - formatted_output.len())
        );
    }

    println!("{}", formatted_output.clone().bold().magenta());
    formatted_output
}

/// Searches for the value associated with a given key in command line arguments.
///
/// Expects arguments in the form `--key value`. Returns the value if the key is found,
/// or an empty `String` if not.
///
/// # Arguments
/// * `key` - A `&str` representing the key to search for.
/// * `argsv` - A `&[String]`, a slice of command line arguments to search through.
///
/// # Returns
/// The value as a `String` if found, otherwise an empty `String`.
///
/// # Example
/// ```
/// let argsv = vec!["--config release".to_string(), "--mode fast".to_string()];
/// let value = search_value("config", &argsv);
/// assert_eq!(value, "release");
/// ```
pub fn search_value(key: &str, argsv: &[String]) -> String {
    let mut counter = 0;

    while counter < argsv.len() {
        if counter + 1 < argsv.len() {
            if argsv[counter].contains(SPLIT_I) {
                let key_name = argsv[counter].replace(SPLIT_I, "");
                if key_name == key {
                    return argsv[counter + 1].to_string();
                }
            }

            if argsv[counter].contains(SPLIT_II) {
                let key_name = argsv[counter].replace(SPLIT_II, "");
                if key_name == key {
                    return argsv[counter + 1].to_string();
                }
            }
        }
        counter += 1;
    }

    raise(
        &format!(
            "search_value :: No value found for {} → Send empty string",
            key
        ),
        "warning",
    );
    String::new()
}

/// Searches for a key in the command line arguments and returns the associated value.
///
/// The function looks for arguments in the format `key=value` and returns the value
/// corresponding to the given key. If the key is not found, it returns an empty `String`.
///
/// # Arguments
/// * `key` - A `&str` representing the key to search for.
/// * `argsv` - A `&[String]`, a slice of command line arguments to search through.
///
/// # Returns
/// A `String` containing the value associated with the key, or an empty `String` if the key is not found.
///
/// # Example
/// ```
/// let argsv = vec!["config=release".to_string(), "mode=fast".to_string()];
/// let value = search_key("config", &argsv);
/// assert_eq!(value, "release");
/// ```
pub fn _search_key(key: &str, argsv: &[String]) -> String {
    for item in argsv {
        if item == key {
            return item.to_string();
        }
    }
    raise(&format!("search_key :: Key not found! {}", key), "warning");
    String::new()
}

/// Formats the input text similar to GNU `fmt`, breaking it into lines with a specified maximum length.
///
/// The function splits the input string into multiple lines, ensuring that no line exceeds
/// the specified `max_length`. It tries to split at word boundaries where possible.
///
/// # Arguments
/// * `input` - A `&str` representing the text to format.
/// * `max_length` - A `usize` specifying the maximum length of each line.
///
/// # Returns
/// A `Vec<String>` where each `String` is a formatted line of the input text.
///
/// # Example
/// ```
/// let input = "This is a sample text that will be formatted to a maximum line length.";
/// let formatted_lines = witch_fmt(input, 20);
/// assert_eq!(formatted_lines, vec![
///     "This is a sample",
///     "text that will be",
///     "formatted to a",
///     "maximum line length."
/// ]);
/// ```
fn witch_fmt(input: &str, max_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let mut end = std::cmp::min(start + max_length, input.len());
        if end < input.len() && !input[end..end + 1].chars().all(char::is_whitespace) {
            if let Some(space_index) = input[..end].rfind(|c: char| c.is_whitespace()) {
                end = space_index + 1;
            }
        }
        if end == start {
            end = std::cmp::min(start + max_length, input.len());
        }
        lines.push(input[start..end].to_string());
        start = end;
    }
    lines
}

/// Generates and prints a help manual page using a predefined database of common command line argument names
/// and their meanings, stored in `core::consts::MAGIC_DOCS`.
///
/// The function retrieves entries from the constant `MAGIC_DOCS`, which is a slice of tuples
/// (`&[(&str, &str)]`), where each tuple consists of a command line argument name and its description.
/// It also uses additional information from `db.json` to provide more detailed explanations.
/// The generated help manual is printed to the console.
///
/// # Returns
/// An `i32` status code indicating success (0) or failure (non-zero).
///
/// # Example
/// ```
/// let status = magic_docs();
/// assert_eq!(status, 0);  // Prints a manual page to the console
/// ```
pub fn magic_docs() -> i32 {
    let data: Vec<DataSet> = data();

    if data.is_empty() {
        raise("magic_docs :: dataset is empty", "fail");
        return 42;
    }

    println!("\n\n");

    fn loop_parser(arg_name: &str) -> Vec<String> {
        for tuple in MAGIC_DOCS {
            if tuple.0 == arg_name.replace("--", "") {
                return witch_fmt(
                    &format!("{}--{} :: {}", " ".repeat(8), tuple.0, tuple.1),
                    120,
                );
            }
        }
        witch_fmt(&format!("{}{}", " ".repeat(8), arg_name), 80)
    }

    for dataset in data {
        let header =
            witch_fmt(&format!("    {} ► {}", dataset.name, dataset.docs), 80).join("\n     ");
        raise(&header, "");

        let mut out: String = dataset.meta.to_string();
        out = out.replace("/", "");
        out = out.replace("\\", "");
        out = out.replace(",", "");
        out = out.replace("'", "");
        out = out.replace("\"", "");
        out = out.replace(";", "");
        out = out.replace("*", "");
        out = out.replace(":", " ");
        out = out.replace("@@@", "@ @@");
        out = out.replace("=@@", " @@");
        let args: Vec<_> = out.split(" ").collect();
        for arg in args {
            if arg.contains("@@") {
                let out = arg.replace(TONK, "--");
                let re = Regex::new(r"^.*?--").unwrap();
                let result = re.replace_all(&out, "--").to_string();

                let lp = loop_parser(&result);
                for item in lp {
                    println!("{}", item);
                }
            }
        }
        println!("");
    }

    return 0;
}

/// Parses a command string and replaces placeholders with values from the provided arguments.
///
/// This function takes a `meta_string` containing an unparsed command string, such as:
/// `hashcat -m 13400 -a 0 @@file @@wordlist`. It searches for specific keys (placeholders)
/// in the `meta_string` that correspond to command line arguments (e.g., `--file`, `--wordlist`).
/// The function automatically replaces these placeholders with their respective values from `argsv`.
///
/// # Arguments
/// * `meta_string` - A `&str` representing the command string to be parsed and modified.
/// * `argsv` - A slice of `String` representing the command line arguments that may contain keys and values.
///
/// # Returns
/// A `String` with the placeholders in `meta_string` replaced by their corresponding values
/// from `argsv`. If any placeholders do not have matching values in `argsv`, they remain unchanged.
///
/// # Example
/// ```
/// let command_template = "hashcat -m 13400 -a 0 @@file @@wordlist";
/// let args = vec![
///     "--file".to_string(), "/path/file.txt".to_string(),
///     "--wordlist".to_string(), "/path/wordlist.txt".to_string(),
/// ];
/// let parsed_command = lazy_parser(command_template, &args);
/// assert_eq!(parsed_command, "hashcat -m 13400 -a 0 /path/file.txt /path/wordlist.txt");
/// ```
///
/// # Note
/// The function assumes that the keys in `argsv` are always preceded by a double hyphen (e.g., `--key`).
pub fn lazy_parser(meta_string: &str, argsv: &[String]) -> String {
    let meta: Vec<&str> = meta_string.split_whitespace().collect();
    let mut cmds: String = meta_string.to_string();

    for item in meta {
        if item.contains("http") {
            let aaaa = item.split("/");
            let mut new = String::new();

            for c in aaaa {
                if c.contains(TONK) {
                    let opt = c.replace(TONK, "");
                    let val = search_value(&opt, argsv);
                    new = item.replace(c, &val);
                }
            }

            cmds = cmds.replace(item, &new);
        }

        if item.contains(TONK) & !item.contains("http") {
            let opt = item.replace(TONK, "");
            let val = search_value(&opt, argsv);
            cmds = cmds.replace(item, &val);
        }
    }

    cmds
}

/// Executes a given command string, such as `"command --arg1 value --arg_url example.com"`.
///
/// The function takes a command line string, parses it, and executes it in the system shell.
/// The command can include arguments and options as part of the string.
/// The function returns the output of the executed command if successful, wrapped in an `Option<Output>`.
/// If execution fails, it returns `None`.
///
/// # Arguments
/// * `command_line` - A `String` representing the full command to execute, including arguments.
///
/// # Returns
/// An `Option<Output>`, where `Some(Output)` contains the command's output if successful, or `None` if execution fails.
///
/// # Example
/// ```
/// let result = raw_exec("ls -la".to_string());
/// if let Some(output) = result {
///     println!("Command executed successfully: {:?}", output);
/// } else {
///     println!("Command execution failed");
/// }
/// ```
///
/// # Note
/// The `Output` type is from the `std::process` module, and represents the output of a child process.
pub fn raw_exec(command_line: String) -> Option<Output> {
    let mut parts = command_line.split_whitespace();
    let command = parts.next().expect("No command found");
    let args: Vec<&str> = parts.collect();

    match Command::new(command).args(&args).output() {
        Ok(output) => Some(output),
        Err(_) => None,
    }
}

/// A wrapper function that executes a command using `raw_exec` and logs the output to a file.
///
/// This function checks for the existence of a `.witchrc` file in the user's home directory.
/// If the file exists and contains the entries `path_log_file="/path/logfile.jsonl"`
/// and `use_log_file=true`, it logs the command's standard output (stdout), standard error (stderr),
/// and exit status to the specified log file.
///
/// # Arguments
/// * `command_line` - A `String` representing the full command to execute, including arguments.
///
/// # Returns
/// An `i32` error code from the executed process. Returns `255` if logging cannot be performed.
///
/// # Example
/// ```
/// let status = lazy_exec("ls -la".to_string());
/// if status == 0 {
///     println!("Command executed successfully.");
/// } else {
///     println!("Command failed with status: {}", status);
/// }
/// ```
///
/// # Note
/// The logging format is in JSON Lines (JSONL), where each line corresponds to a log entry.
pub fn lazy_exec(command_line: String) -> i32 {
    match raw_exec(command_line.clone()) {
        Some(output) => {
            if !core_logger(&output, &command_line) {
                if SW_DEBUG {
                    raise(
                        "Logger are enabled, but the log files are not being created",
                        "fail",
                    );
                }
            }

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines = stdout.split("\n");
                for line in lines {
                    let result = witch_fmt(line, 180);
                    for line in result {
                        println!("\t{}", line);
                        io::stdout().flush().unwrap();
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                raise(&format!("{}", stderr.to_string()), "fail");
            }
            println!(" ");
            output.status.code().unwrap_or(128)
        }
        None => 255,
    }
}

/// An implementation of `lazy_exec` tailored for the `DataSet`, allowing execution of any command line string
/// within the context of the provided dataset, returning the standard exit status.
///
/// This function utilizes the fields of the `DataSet` structure to customize the command execution,
/// enabling it to work with various command line inputs. It takes a `DataSet` and a slice of command line
/// arguments, executes the command, and returns the exit status.
///
/// # Arguments
/// * `set` - A `DataSet` instance containing relevant documentation, name, and metadata for execution context.
/// * `argsv` - A slice of `String` representing the command line arguments to be executed.
///
/// # Returns
/// An `i32` representing the exit status of the executed command. A value of 0 indicates success,
/// while any other value indicates failure.
///
/// # Example
/// ```
/// let dataset = DataSet {
///     docs: "Sample Command documentation".to_string(),
///     name: "sample.command".to_string(),
///     meta: "ls -lha".to_string(),
/// };
/// let status = flawless_exec(dataset, &["sample.command".to_string()]);
/// println!("Exit status: {}", status); // Prints the exit status of the command
/// ```
///
/// # Note
/// Ensure that the command being executed is valid and that the dataset provides necessary context
/// for execution. Also the command name shoud be in lowercase.
pub fn flawless_exec(set: DataSet, argsv: &[String]) -> i32 {
    raise(&set.name, "");
    let cmd = lazy_parser(&set.meta, argsv);
    lazy_exec(cmd)
}

// Executes a command within the context of a specified dataset from the `DataSet` collection.
///
/// This function serves as a specialized implementation of `lazy_exec` for `DataSet` operations.
/// It searches for a dataset matching the provided name in the first argument and either:
/// - Executes the command in the context of the matched dataset using `flawless_exec`, or
/// - Falls back to `lazy_exec` with the remaining arguments if no match is found
///
/// # Arguments
/// * `argsv` - A slice of Strings containing:
///   - The first element should be the dataset name to match against
///   - Remaining elements form the command to execute
///
/// # Returns
/// The exit status code from either:
/// - `flawless_exec` if a dataset match is found
/// - `lazy_exec` if no dataset match is found
/// - 42 if no datasets are available (unreachable if data() always returns non-empty)
///
/// # Behavior
/// - Uses the fields of the matched `DataSet` structure to customize command execution
/// - Joins remaining arguments with spaces when falling back to `lazy_exec`
/// - Returns immediately upon finding first matching dataset
pub fn flawless_entry_point(argsv: &[String]) -> i32 {
    let mname = argsv[1].as_str();
    let data = data();
    for set in data {
        if set.name == mname {
            return flawless_exec(set.clone(), argsv);
        }
    }

    if witchy_readrc_value("exec_external_command") == "yes" {
        if SW_DEBUG {
            raise("Exec external command is enabled", "warning");
        }
        return lazy_exec(argsv[1..].join(" "));
    }

    255
}

/// Recursively collects the paths of all files in a given directory and its subdirectories.
///
/// This function takes a reference to a `Path`, traverses the directory, and returns
/// a vector of strings representing the paths of all files found. It includes files
/// from subdirectories as well.
///
/// # Arguments
/// * `dir` - A reference to a `Path` representing the directory to search.
///
/// # Returns
/// A `Vec<String>` containing the paths of all files found in the specified directory
/// and its subdirectories.
///
/// # Example
/// ```
/// use std::path::Path;
///
/// let files = directory_lookup(Path::new("/path/to/directory"));
/// for file in files {
///     println!("{}", file); // Prints each file's path
/// }
/// ```
pub fn directory_lookup(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(directory_lookup(&path));
        } else {
            files.push(path.to_string_lossy().to_string());
        }
    }

    files
}

/// Executes a CommandRegistry shell with a custom set of commands defined by a `CommandRegistry` type.
///
/// The function takes a vector of tuples, where each tuple consists of a command name
/// and a corresponding function that implements the command logic. It executes the
/// command associated with the provided arguments and returns the standard exit status.
///
/// # Arguments
/// * `options` - A `CommandRegistry`, which is a vector of tuples. Each tuple contains:
///     - A `&'static str` representing the command name.
///     - A function pointer (`fn(&[String]) -> i32`) that executes the command logic.
/// * `argsv` - A slice of `String` representing the arguments to pass to the command function.
///
/// # Returns
/// An `i32` representing the exit status of the executed command. A return value of 0 indicates
/// success, while any other value indicates failure.
///
/// # Example
/// ```
/// type CommandRegistry = Vec<(&'static str, fn(&[String]) -> i32)>;
///
/// fn command_function(argsv: &[String]) -> i32 {
///     // Command implementation
///     println!("Command executed with arguments: {:?}", argsv);
///     0 // Indicate success
/// }
///
/// let options: CommandRegistry = vec![("command.name", command_function)];
/// let status = command_shell(options, &["arg1".to_string(), "arg2".to_string()]);
/// println!("Exit status: {}", status); // Prints the exit status of the command
/// ```
pub fn command_shell(options: CommandRegistry, argsv: &[String]) -> i32 {
    if argsv.len() % 2 != 0 {
        println!("{}", WITCH);
        println!("{}", BOTTOM_TEXT);
        return 0;
    }

    let name = argsv[1].as_str();

    for function in options {
        if function.0 == name {
            return function.1(argsv);
        }
    }

    return 8080;
}

/// Reads the contents of a file and returns its lines as a `Vec<String>`.
///
/// This function opens the specified file, reads its contents, and splits it into lines,
/// returning each line as a separate `String` in a vector. If the file cannot be read,
/// it returns an empty vector.
///
/// # Arguments
/// * `path` - A `&str` representing the path to the file to read.
///
/// # Returns
/// A `Vec<String>` containing the lines read from the file. If the file cannot be read,
/// an empty vector is returned.
///
/// # Example
/// ```
/// let lines = read_file_to_lines("example.txt");
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
pub fn read_file_to_lines(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(value) => value.lines().map(String::from).collect(),
        Err(err) => {
            raise(&format!("Error at {}", err), "message");
            return Vec::new();
        }
    }
}

/// Converts an IP address string to its decimal representation.
///
/// This function takes a string representation of an IP address (IPv4 or IPv6),
/// converts it to a decimal number, and returns the result as a `String`.
/// If the conversion fails (e.g., due to an invalid IP format), it returns an empty string.
///
/// # Arguments
/// * `ip_str` - A `&str` representing the IP address to convert.
///
/// # Returns
/// A `String` containing the decimal representation of the IP address. If the conversion fails,
/// an empty `String` is returned.
///
/// # Example
/// ```
/// let decimal_ip = ip_to_number("192.168.1.1");
/// assert_eq!(decimal_ip, "3232235777"); // Decimal representation of "192.168.1.1"
///
/// let invalid_ip = ip_to_number("invalid_ip");
/// assert_eq!(invalid_ip, ""); // Returns an empty string for invalid input
/// ```
pub fn ip_to_number(ip_str: &str) -> String {
    fn convert_addr(ip: IpAddr) -> u128 {
        match ip {
            IpAddr::V4(ipv4) => u128::from(u32::from_be_bytes(ipv4.octets())),
            IpAddr::V6(ipv6) => {
                let bytes = ipv6.octets();
                let mut num = 0u128;
                for byte in bytes {
                    num = (num << 8) | u128::from(byte);
                }
                num
            }
        }
    }

    match IpAddr::from_str(ip_str) {
        Ok(ip) => {
            let number = convert_addr(ip);
            number.to_string()
        }
        Err(e) => {
            println!("Failed to parse IP address '{}': {}", ip_str, e);
            String::new()
        }
    }
}

/// Searches the operating system environment for a given key and returns its value.
///
/// This function looks up the specified environment variable key in the OS environment.
/// If the key exists, it returns the associated value as a `String`. If the key is not found,
/// it returns an empty string.
///
/// # Arguments
/// * `key` - A `&str` representing the name of the environment variable to search for.
///
/// # Returns
/// A `String` containing the value of the environment variable if found, or an empty string if not.
///
/// # Example
/// ```
/// let path = get_os_env("PATH");
/// println!("PATH: {}", path); // Prints the value of the PATH environment variable
///
/// let non_existent = get_os_env("NON_EXISTENT_ENV");
/// assert_eq!(non_existent, ""); // Returns an empty string for a non-existent key
/// ```
pub fn get_os_env(key: &str) -> String {
    match env::var_os(key) {
        Some(val) => format!("{:?}", val).replace("\"", ""),
        None => {
            // raise("Env key not found", "fail");
            String::new()
        }
    }
}

// A variation of get_os_env function that handle better the home variable.
pub fn get_os_env_paths_only(path: &str) -> String {
    let largatixa = get_os_env(path).to_string();

    if largatixa.is_empty() {
        return "./".to_string();
    }

    if largatixa.ends_with("/") {
        return largatixa;
    }
    return format!("{}/", largatixa);
}

/// Show witchcraft software version!
pub fn show_version() -> i32 {
    raise(VERSION, "");
    return 0;
}
