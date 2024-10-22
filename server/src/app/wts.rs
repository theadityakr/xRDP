use windows::Win32::System::RemoteDesktop::{WTSEnumerateSessionsW, WTSQueryUserToken, WTS_SESSION_INFOW, WTS};
use windows::Win32::System::Threading::{CreateProcessAsUserW};
use windows::Win32::Foundation::{HANDLE, BOOL};
use windows::Win32::Security::LogonUserW;
use winapi::um::handleapi::CloseHandle;
use std::ptr::null_mut;

pub fn start_wts_session(username: &str, domain: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Enumerate sessions to get a valid session ID
        let mut session_count: u32 = 0;
        let mut session_info_ptr: *mut WTS_SESSION_INFOW = null_mut();
        if WTSEnumerateSessionsW(null_mut(), 0, 1, &mut session_info_ptr, &mut session_count).as_bool() {
            let session_info = std::slice::from_raw_parts(session_info_ptr, session_count as usize);
            for session in session_info {
                println!("Session ID: {}", session.SessionId);
            }
        }

        // Log on the user using LogonUserW (domain, username, and password needed)
        let mut token: HANDLE = HANDLE::default();
        let success = LogonUserW(
            username,
            domain,
            password,
            2,  // LOGON32_LOGON_INTERACTIVE
            0,  // LOGON32_PROVIDER_DEFAULT
            &mut token
        );

        if !success.as_bool() {
            return Err("Failed to log on the user.".into());
        }

        // Query user token to start a process in the session
        let mut user_token: HANDLE = HANDLE::default();
        if !WTSQueryUserToken(session_info[0].SessionId, &mut user_token).as_bool() {
            return Err("Failed to query user token.".into());
        }

        // Launch a process in the user’s session using CreateProcessAsUserW
        let mut process_info = std::mem::zeroed();
        let startup_info = std::mem::zeroed();
        let success = CreateProcessAsUserW(
            user_token,
            None,  // No application name (just executable)
            Some("C:\\Windows\\System32\\notepad.exe"),  // Example process
            null_mut(),  // Default security attributes
            null_mut(),
            false,
            0,
            null_mut(),
            None,
            &startup_info,
            &mut process_info
        );

        if success.as_bool() {
            println!("Process created successfully.");
        } else {
            println!("Failed to create process.");
        }

        // Close handles
        CloseHandle(token);
        CloseHandle(user_token);
    }

    Ok(())
}
