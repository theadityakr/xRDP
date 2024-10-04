use serde::{Serialize, Deserialize};
use std::error::Error;
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::{DESKTOP_CREATEMENU, DESKTOP_CREATEWINDOW, DESKTOP_WRITEOBJECTS, DESKTOP_SWITCHDESKTOP};
use winapi::um::winbase::{LogonUserW, LOGON32_LOGON_INTERACTIVE, LOGON32_PROVIDER_DEFAULT};

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    address: String,
    username: String,
    password: String,
}



fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

pub fn authenticate_user(credentials: Credentials) -> Result<HANDLE, Box<dyn Error>> {

    let username = to_wide_chars(&credentials.username);
    let password = to_wide_chars(&credentials.password);
    let domain = to_wide_chars(&credentials.address);

    let mut token: HANDLE = null_mut();
    
    let result = unsafe {
        LogonUserW(
            username.as_ptr(),
            domain.as_ptr(),
            password.as_ptr(),
            LOGON32_LOGON_INTERACTIVE,
            LOGON32_PROVIDER_DEFAULT,
            &mut token
        )
    };

    if result == 0 {
        return Err("Authentication failed".into());
    }

    Ok(token)
}

  