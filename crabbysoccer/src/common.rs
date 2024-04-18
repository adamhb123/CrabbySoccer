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
