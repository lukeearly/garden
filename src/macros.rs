#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => (println!("\x1b[35;40mVerbose:\x1b[0m {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (println!("\x1b[34;40mDebug:\x1b[0m {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (println!("\x1b[32;40mInfo:\x1b[0m {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (println!("\x1b[30;43mWarn:\x1b[0m {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (println!("\x1b[30;41mError:\x1b[0m {}", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! crit {
    ($($arg:tt)*) => (println!("\x1b[31;47mCritical Failure:\x1b[0m {}", format_args!($($arg)*)));
}
