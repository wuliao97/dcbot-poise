pub mod activity;
pub mod describe;
pub mod embed;
pub mod page;
pub mod button;


#[macro_export]
macro_rules! quote {
    ($target:expr, $($arg:tt)+) => {
        format!(">>> {}", format!($target,  $($arg)+))
    };

    ($($arg:expr)+) => {
        format!(">>> {}", format!("{}", $($arg)+))
    };
}

#[macro_export]
macro_rules! quote_with_bold {
    ($target:expr, $($arg:tt)+) => {
        format!(">>> **{}**", format!($target,  $($arg)+))
    };

    ($($arg:expr)+) => {
        format!(">>> **{}**", format!("{}", $($arg)+))
    };
}


#[macro_export]
macro_rules! url {
    ($target:expr, $arg:expr) => {
        format!("[{}]({})", $target,  $arg)
    };
}


#[macro_export]
macro_rules! url_with_bold {
    ($target:expr, $arg:expr) => {
        format!("**[{}]({})**", $target,  $arg)
    };
}

