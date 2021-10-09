use std::{
    fmt::Display,
    process,
};

pub fn log_msg<T: Display>(msg: T) {
    println!("\x1b[1m[LOG]\x1b[0m {}", msg);
}

pub fn log_warning<T: Display>(warning: T) {
    println!("\x1b[33;1m[WARNING]\x1b[0;33m {}\x1b[0m", warning);
}

pub fn log_error<T: Display>(error: T) {
    println!("\x1b[31;1m[ERROR]\x1b[0;31m {}\x1b[0m", error);
}

pub fn log_fatal<T: Display>(error: T) -> ! {
    println!("\x1b[31;1m[FATAL]\x1b[0;31m {}\x1b[0m", error);
    process::exit(1);
}
