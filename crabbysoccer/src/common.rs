use itertools;

pub fn format_vec<T: ToString>(fmt_str: &str, args: &Vec<T>) -> String {
    let args: Vec<String> = args.iter().map(|e| e.to_string()).collect();
    let pieces: Vec<String> = fmt_str.split("{}").map(|e| e.to_string()).collect();
    itertools::interleave(pieces, args).collect()
}
