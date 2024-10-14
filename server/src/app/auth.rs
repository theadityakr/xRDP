use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use std::error::Error;
use windows::Win32::Foundation::{HANDLE, BOOL};
use winapi::um::handleapi::CloseHandle;
use windows::Win32::Security::{
    LogonUserW,
    LOGON32_LOGON_INTERACTIVE,
    LOGON32_PROVIDER_DEFAULT,
};
use windows::core::{PCWSTR, w};


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

fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub async fn authenticate_user(connection_settings: String) -> Result<(HANDLE, String), Box<dyn Error>> {
    let credentials = initial_check(connection_settings).await?;
    let username = to_wide_string(&credentials.username);
    let password = to_wide_string(&credentials.password);
    let domain = to_wide_string(&credentials.address);

    let mut token = HANDLE::default();
    
    let result = unsafe {
        LogonUserW(
            PCWSTR::from_raw(username.as_ptr()),
            PCWSTR::from_raw(domain.as_ptr()),
            PCWSTR::from_raw(password.as_ptr()),
            LOGON32_LOGON_INTERACTIVE,
            LOGON32_PROVIDER_DEFAULT,
            &mut token
        )
    };

    if result.as_bool() {
        Ok((token, credentials.username))
    } else {
        Err("Authentication failed".into())
    }
}
