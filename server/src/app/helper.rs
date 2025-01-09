use get_last_error::Win32Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum Status {
    Error,
    Success,
    Debug,
}

pub mod wts_log {
    use super::{Status, Win32Error};
    use std::fmt::Debug;

    // Log error and return Win32Error
    pub fn error(function: &str, message: &str) -> Win32Error {
        let last_error = Win32Error::get_last_error();
        eprintln!(
            "[{}] [Error] {}: {}",
            function, message, last_error
        );
        last_error
    }

    pub fn success(function: &str, message: &str, value: Option<&str>) {
        let value = value.unwrap_or("");
        eprintln!(
            "[{}] [Success] {} : {}",
            function, message, value
        );
    }

    pub fn debug<T: Debug>(function: &str, message: &str, value: Option<T>) {
        match value {
            Some(v) => {
                eprintln!(
                    "[{}] [Debug] {} : {:?}",
                    function, message, v
                );
            }
            None => {
                eprintln!(
                    "[{}] [Debug] {}",
                    function, message
                );
            }
        }
    }
}

pub mod server_log {
    use super::{Status, Win32Error};
    use std::fmt::Debug;

    pub fn error<T: Debug>(function: &str, message: &str, error: Option<T>) {
        match error {
            Some(v) => {
                eprintln!(
                    "[{}] [Error] {} : {:?}",
                    function, message, v
                );
            }
            None => {
                eprintln!(
                    "[{}] [Error] {}",
                    function, message
                );
            }
        }
    }

    pub fn success(function: &str, message: &str, value: Option<&str>) {
        let value = value.unwrap_or("");
        eprintln!(
            "[{}] [Success] {} : {}",
            function, message, value
        );
    }

    pub fn debug<T: Debug>(function: &str, message: &str, value: Option<T>) {
        match value {
            Some(v) => {
                eprintln!(
                    "[{}] [Debug] {} : {:?}",
                    function, message, v
                );
            }
            None => {
                eprintln!(
                    "[{}] [Debug] {}",
                    function, message
                );
            }
        }
    }
}