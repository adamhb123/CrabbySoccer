#[allow(dead_code)]
pub fn format_vec<T: ToString>(fmt_str: &str, args: &Vec<T>) -> String {
    let args: Vec<String> = args.iter().map(|e| e.to_string()).collect();
    let mut fmt_str = fmt_str.to_owned();
    for i in 0..args.len() {
        // Replace index-based placeholders to appropriate strings
        fmt_str = fmt_str.replace(format!("{{{i}}}").as_str(), args[i].as_str());
        // Replace plain placeholder
        fmt_str = fmt_str.replacen("{}", args[i].as_str(), 1);
    }
    fmt_str
}
