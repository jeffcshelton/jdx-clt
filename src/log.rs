#[macro_export]
macro_rules! log_msg {
    ($msg: expr) => {
        println!("\x1b[1m[LOG]\x1b[0m {}", $msg);
    };
}

#[macro_export]
macro_rules! log_warning {
    ($warning: expr) => {
        println!("\x1b[33;1m[WARNING]\x1b[0;33m {}\x1b[0m", $warning);
    };
}

#[macro_export]
macro_rules! log_error {
    ($error: expr) => {
        println!("\x1b[31;1m[ERROR]\x1b[0;31m {}\x1b[0m", $error);
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($error: expr) => {
        println!("\x1b[31;1m[FATAL]\x1b[0;31m {}\x1b[0m", $error);
        std::process::exit(1);
    };
}
