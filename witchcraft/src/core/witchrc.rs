use crate::core::core::get_os_env_paths_only;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Checks if the configuration file `.witchrc` exists in the user's home directory.
///
/// This function constructs the path to the `.witchrc` file located in the user's home directory
/// by appending `.witchrc` to the value of the `HOME` environment variable. It then checks
/// for the existence of the file using the `fs::metadata` function.
///
/// # Returns
/// A `bool` indicating whether the `.witchrc` file exists (`true`) or not (`false`).
///
/// # Example
/// ```
/// if rc_exists() {
///     println!("The configuration file '.witchrc' exists.");
/// } else {
///     println!("The configuration file '.witchrc' does not exist.");
/// }
/// ```
///
/// # Note
/// The function relies on the presence of the `HOME` environment variable to locate the user's
/// home directory. If the environment variable is not set, the behavior may be undefined.
pub fn rc_exists() -> bool {
    let home = format!("{}.witchrc", get_os_env_paths_only("HOME"));
    fs::metadata(home).is_ok()
}

/// Checks if a specific key exists in the configuration file `~/.witchrc`.
///
/// This function looks for a line in the `.witchrc` file that contains the specified `key`
/// in the format `key=value`. It returns `true` if the key exists, and `false` otherwise.
///
/// # Arguments
/// * `key` - A `&str` representing the key to search for in the `.witchrc` file.
///
/// # Returns
/// A `bool` indicating whether the specified key exists in the configuration file.
///
/// # Example
/// ```
/// let exists = readrc_exists("path_log_file");
/// if exists {
///     println!("Key exists in the configuration file.");
/// } else {
///     println!("Key does not exist.");
/// }
/// ```
#[allow(dead_code)]
pub fn witchy_if_rc_key_exists(key: &str) -> bool {
    let fkey = format!("{}=", key);

    if !rc_exists() {
        "error".to_string();
    }

    match fs::read_to_string("~/.witchrc") {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return true;
                }
            }
            false
        }
        Err(err) => {
            eprintln!("{}", err);
            return false;
        }
    }
}

/// Retrieves the value associated with a specific key from the configuration file `~/.witchrc`.
///
/// This function searches the `.witchrc` file for a line containing the specified `key` in
/// the format `key=value`. If the key is found, it returns the corresponding value as a `String`.
/// If the key does not exist or if the file cannot be read, it returns an empty string or
/// the string `"error"` respectively.
///
/// # Arguments
/// * `key` - A `&str` representing the key for which to retrieve the value from the `.witchrc` file.
///
/// # Returns
/// A `String` containing the value associated with the specified key if found. Returns an
/// empty string if the key does not exist, or `"error"` if the file cannot be read.
///
/// # Example
/// ```
/// let value = readrc_value("path_log_file");
/// if value.is_empty() {
///     println!("Key not found or has no value.");
/// } else {
///     println!("Value: {}", value);
/// }
/// ```
///
/// # Note
/// This function assumes that the `.witchrc` file is located in the user's home directory.
/// If the file cannot be read, an error message is printed along with the file path and an empty string
/// are returned
pub fn witchy_readrc_value(key: &str) -> String {
    let fkey = format!("{}=", key);
    let home = format!("{}.witchrc", get_os_env_paths_only("HOME"));

    if !rc_exists() {
        "error".to_string();
    }

    match fs::read_to_string(&home) {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return line.replace(&fkey, "");
                }
            }
            "".to_string()
        }
        Err(err) => {
            println!("{} :: {}", err, home);
            return "".to_string();
        }
    }
}

/// Creates or updates a `.witchrc` configuration file in the user's home directory.
///
/// This function checks if the `.witchrc` file already exists using the `rc_exists` function.
/// If the file exists, it appends a new configuration line to the file. If the file does not exist,
/// it creates the file and writes the configuration line.
///
/// The configuration line added to the file is:
/// ```plaintext
/// path_log_file=~/witchcraft_smartlog.jsonl
/// ```
///
/// # Returns
/// - `false`: This function always returns `false`. The return value is currently not used
///   and may be updated in future versions to indicate success or failure.
///
/// # Panics
/// - This function will panic if it fails to open or write to the `.witchrc` file.
///
/// # Example
/// ```rust
/// let result = create_rc_default();
/// assert_eq!(result, false); // Currently always returns `false`
/// ```
pub fn create_rc_default(_argsv: &[String]) -> i32 {
    if !rc_exists() {
        let home = get_os_env_paths_only("HOME");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&format!("{}.witchrc", home))
            .unwrap();
        writeln!(file, "path_log_file=~/witchcraft_smartlog.jsonl").unwrap();
    }

    0
}
