#[macro_export]
macro_rules! crabby_println {
    () => {
        println!("[crabbysoccer]");
    };
    ($($arg:tt)*) => {
        println!("{}", $($arg)*);
    };
}
#[macro_export]
macro_rules! warn {
    () => {
        println!("WARNING".red());
    };
    ($fmt:expr $(, $($arg:tt)*)?) => { println!("{}", format!("[crabbysoccer] WARNING: {}", $($($arg)*)?)) };
}