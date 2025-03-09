pub type CommandRegistry = Vec<(&'static str, fn(&[String]) -> i32)>;
