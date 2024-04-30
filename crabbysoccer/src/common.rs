use std::{fmt::Display, io::Write};

#[allow(dead_code)]
pub fn format_vec<T: ToString>(fmt_str: &str, args: &[T]) -> String {
    let args: Vec<String> = args.iter().map(|e| e.to_string()).collect();
    let mut fmt_str = fmt_str.to_owned();
    for (i, arg) in args.iter().enumerate() {
        // Replace index-based placeholders to appropriate strings
        fmt_str = fmt_str.replace(format!("{{{i}}}").as_str(), arg.as_str());
        // Replace plain placeholder
        fmt_str = fmt_str.replacen("{}", arg.as_str(), 1);
    }
    fmt_str
}

pub fn println_then_show_input_indicator<T: Display>(val: T) {
    println!("\r{}", val);
    std::io::stdout().flush().unwrap();
    print!("$ ");
    std::io::stdout().flush().unwrap();
}

#[derive(Copy, Clone)]
pub enum InputAction {
    Quit,
    Help,
    ListConnections,
}

pub const INPUT_ACTION_PARSE_DEFS: [((&str, &str), InputAction); 3] = [
    // Format: ((INPUT_PATTERN, INPUT_PATTERN_SHORTHAND), InputAction::{})
    (("quit", "q"), InputAction::Quit),
    (("help", "h"), InputAction::Help),
    (("list-connections", "lc"), InputAction::ListConnections),
];

pub fn parse_input_action<T: ToString>(args: &[T]) -> Option<InputAction> {
    let args: Vec<String> = args.iter().map(|e| e.to_string()).collect();
    INPUT_ACTION_PARSE_DEFS
        .iter()
        .find(|((pat, short_pat), _)| args[0].contains(pat) || args[0] == *short_pat)
        .map(|e| e.1)
}
