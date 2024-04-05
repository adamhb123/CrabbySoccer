use itertools::{self, Itertools};

pub fn format_vec<T: ToString>(fmt_str: &str, args: &Vec<T>) -> String {
    let args: Vec<String> = args.iter().map(|e| e.to_string()).collect();
    // Replace index-based placeholders to appropriate strings
    let mut fmt_str = fmt_str.to_string();
    (0..args.len()).for_each(|i| {fmt_str = fmt_str.replace(format!("{{{i}}}").as_str(), args[i].as_str()); });
    // Replace plain placeholders
    let pieces: Vec<String> = fmt_str.split("{}").map(|e| e.to_string()).collect();
    pieces.into_iter().interleave_shortest(args).collect()
}
