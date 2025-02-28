use super::witchrc::witchy_readrc_value;
use crate::core::core::*;
use crate::core::witchrc::rc_exists;
use crate::datetime_now;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::OpenOptions, process::Output};

/// A utility struct for logging command-related information.
///
/// This struct stores command output, status, errors, and additional metadata.
/// It supports serialization and deserialization via `serde` and provides methods
/// for creating, saving, and converting the log data to JSON.
#[derive(Serialize, Deserialize)]
pub struct WitchyLogger {
    cmd_output: String,
    cmd_status: String,
    cmd_error: String,
    cmd_true: String,
    cmd_witchy: String,
    datetime_now: String,
}

impl WitchyLogger {
    /// Creates a new instance of `WitchyLogger` with the provided command-related data.
    ///
    /// The `datetime_now` field is automatically populated with the current date and time.
    ///
    /// # Arguments
    /// * `cmd_output` - The output of the command.
    /// * `cmd_status` - The status of the command.
    /// * `cmd_error` - Any error messages associated with the command.
    /// * `cmd_true` - Additional command-related data.
    /// * `cmd_witchy` - Witchy-specific command metadata.
    ///
    /// # Returns
    /// A new `WitchyLogger` instance.
    pub fn new(
        cmd_output: String,
        cmd_status: String,
        cmd_error: String,
        cmd_true: String,
        cmd_witchy: String,
    ) -> Self {
        WitchyLogger {
            cmd_output,
            cmd_status,
            cmd_error,
            cmd_true,
            cmd_witchy,
            datetime_now: datetime_now(),
        }
    }

    /// Creates an empty instance of `WitchyLogger` with all fields initialized to empty strings.
    ///
    /// The `datetime_now` field is automatically populated with the current date and time.
    ///
    /// # Returns
    /// An empty `WitchyLogger` instance.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        WitchyLogger {
            cmd_output: String::new(),
            cmd_status: String::new(),
            cmd_error: String::new(),
            cmd_true: String::new(),
            cmd_witchy: String::new(),
            datetime_now: datetime_now(),
        }
    }

    /// Serializes the `WitchyLogger` instance to a JSON string.
    ///
    /// # Returns
    /// A JSON string representation of the `WitchyLogger` instance.
    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Saves the `WitchyLogger` instance to a log file.
    ///
    /// The file path is determined by reading the `path_log_file` value from the Witchy
    /// configuration file (`witchrc`) and resolving it relative to the user's home directory.
    ///
    /// # Behavior
    /// 1. Serializes the `WitchyLogger` instance to a JSON string.
    /// 2. Reads the `path_log_file` value from the Witchy configuration file.
    /// 3. Resolves the path by replacing `~/` with the user's home directory.
    /// 4. Opens the file in append mode (creates it if it doesn't exist).
    /// 5. Writes the JSON string to the file.
    /// 6. If the file cannot be opened or written to, an error is raised.
    ///
    /// # Returns
    /// The JSON string that was saved to the file. If the file path or home directory
    /// cannot be determined, an empty string is returned.
    pub fn save(&self) -> String {
        let output = serde_json::to_string(self).unwrap();
        let witchrc = witchy_readrc_value("path_log_file");
        let home = get_os_env_paths_only("HOME");

        if witchrc.is_empty() || home.is_empty() {
            return String::new();
        }

        let path = witchrc.replace("~/", &home);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path);

        match file {
            Ok(mut file) => writeln!(file, "{}", output).unwrap(),
            Err(err) => {
                raise(
                    &format!("WitchyLogger :: {} | path :: {}", err.to_string(), path),
                    "fail",
                );
            }
        };

        output
    }
}

/// A wrapper function that logs command execution details using `WitchyLogger`.
///
/// This function takes an `Output` object from a command execution and the command line string
/// that was executed. It uses the `WitchyLogger` struct to log the output, including standard
/// output, standard error, and the exit status of the command. This is primarily used in
/// conjunction with the `lazy_exec` function.
///
/// # Arguments
/// * `output` - An `Output` object containing the result of the command execution, including
///   `stdout`, `stderr`, and the exit status.
/// * `command_line` - A `&String` representing the command line string that was executed.
///
/// # Returns
/// A `bool` indicating whether the logging operation was successful (`true`) or not (`false`).
///
/// # Example
/// ```
/// let command_output = Output {
///     stdout: b"Command executed successfully".to_vec(),
///     stderr: b"".to_vec(),
///     status: ExitStatus::from_raw(0)
/// };
/// let command_line = "example.command --arg".to_string();
///
/// if core_logger(&command_output, &command_line) {
///     println!("Logging successful.");
/// } else {
///     println!("Logging failed.");
/// }
/// ```
///
/// # Note
/// Ensure that the `WitchyLogger` struct is properly initialized before calling this function.
/// The logging functionality is intended to capture and store the output of command executions
/// for debugging and record-keeping purposes.
pub fn core_logger(output: &Output, command_line: &String) -> bool {
    if rc_exists() == false {
        return false;
    }

    let logger = WitchyLogger::new(
        String::from_utf8_lossy(&output.stdout).to_string(),
        output
            .status
            .code()
            .unwrap_or(156)
            .to_string()
            .to_owned()
            .clone(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        command_line.to_string(),
        String::from("Some witchy details"),
    );

    if logger.save().is_empty() {
        return false;
    }

    return true;
}

/// Logs command execution details using the `WitchyLogger`.
///
/// This function takes various command-related outputs and logs them using a `WitchyLogger`
/// instance. If the required configuration (`rc_exists()`) is missing, the function returns `true`
/// immediately, indicating that logging is skipped. Otherwise, it attempts to save the log and
/// returns `true` if successful, or `false` if the log could not be saved.
///
/// # Arguments
///
/// * `cmd_output` - The standard output of the executed command.
/// * `cmd_status` - The status of the command execution.
/// * `cmd_error` - Any error messages produced during execution.
/// * `cmd_true` - A reference to a command line it self.
/// * `cmd_witchy` - Additional commentary or metadata for logging.
///
/// # Returns
///
/// * `true` if logging was either skipped (due to missing configuration) or successfully saved.
/// * `false` if the logging operation failed.
///
/// # Example
///
/// ```rust
/// let success = doglog("output", "status", "error", "true_val", "metadata");
/// assert!(success);
/// ```
#[allow(dead_code)]
pub fn doglog(
    cmd_output: &str,
    cmd_status: &str,
    cmd_error: &str,
    cmd_true: &str,
    cmd_witchy: &str,
) -> bool {
    if rc_exists() == false {
        return true;
    }

    let logger = WitchyLogger::new(
        cmd_output.to_string(),
        cmd_status.to_string(),
        cmd_error.to_string(),
        cmd_true.to_string(),
        cmd_witchy.to_string(),
    );

    if logger.save().is_empty() {
        return false;
    }

    return true;
}
