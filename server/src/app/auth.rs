use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use std::error::Error;
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::{DESKTOP_CREATEMENU, DESKTOP_CREATEWINDOW, DESKTOP_WRITEOBJECTS, DESKTOP_SWITCHDESKTOP};
use winapi::um::winbase::{LogonUserW, LOGON32_LOGON_INTERACTIVE, LOGON32_PROVIDER_DEFAULT};
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ConnectionSettings {
    computer: String,
    username: String,
    password: String,
    general_settings: GeneralSettings,
    advanced_settings: AdvancedSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeneralSettings {
    save_password: bool,
    multiple_display: bool,
    local_drives_redirection: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AdvancedSettings {
    printers: bool,
    clipboard: bool,
}

#[derive(Debug)]
struct Credentials {
    address: String,
    username: String,
    password: String,
}

async fn initial_check(connection_settings: String) -> JsonResult<Credentials> {
    let settings: ConnectionSettings = serde_json::from_str(&connection_settings)?;
    
    Ok(Credentials {
        address: settings.computer,
        username: settings.username,
        password: settings.password
    })
}

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

pub async fn authenticate_user(connection_settings: String) -> Result<HANDLE, Box<dyn Error>> {

    let credentials = initial_check(connection_settings).await?;
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

    println!("{:?}",&token);
    if result == 0 {
        return Err("Authentication failed".into());
    }

    Ok(token)
}

  