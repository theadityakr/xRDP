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
use std::ffi::{CStr, OsStr};
use windows::Win32::System::SystemInformation::{ComputerNameDnsDomain, GetComputerNameExA};
use windows::core::{PCWSTR, w,PWSTR, PSTR};

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

fn get_domain_name() -> Vec<u16> {
    let mut buffer: [u8; 256] = [0; 256];
    let mut size = buffer.len() as u32;

    let result = unsafe {
        GetComputerNameExA(
            ComputerNameDnsDomain,
            PSTR::from_raw(buffer.as_mut_ptr()), // Cast to PSTR
            &mut size,
        )
    };

    if result.as_bool() {
        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
        let domain_name = cstr.to_string_lossy().to_string();
        to_wide_string(&domain_name)
    } else {
        vec![0u16]
    }
}

pub async fn authenticate_user(connection_settings: String) -> Result<(HANDLE, String), Box<dyn Error>> {
    let credentials = initial_check(connection_settings).await?;
    let username = to_wide_string(&credentials.username);
    let password = to_wide_string(&credentials.password);
    let mut token = HANDLE::default();
    let domain = get_domain_name();
    println!("[authenticate_user] [Debug] [domain]: {:?}",domain);

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
