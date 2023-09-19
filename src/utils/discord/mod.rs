pub mod activity;
pub mod describe;


#[macro_export]
macro_rules! quote {
    ($target:expr, $($arg:tt)+) => {
        format!(">>> {}", format!($target,  $($arg)+))
    };

    ($($arg:tt)+) => {
        format!(">>> {}", format!("{}", $($arg)+))
    };
}


#[macro_export]
macro_rules! url {
    ($target:tt, $arg:tt) => {
        format!("[{}]({})", $target,  $arg)
    };
}


#[macro_export]
macro_rules! url_with_bold {
    ($target:tt, $arg:tt) => {
        format!("**[{}]({})**", $target,  $arg)
    };
}
