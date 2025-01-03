use windows::Win32::System::RemoteDesktop::{
    WTSOpenServerA, WTSCloseServer, WTSQuerySessionInformationA,
    WTSEnumerateSessionsA, WTS_SESSION_INFOA, WTSActive,
    WTS_CURRENT_SESSION, WTS_INFO_CLASS
};
use get_last_error::Win32Error;
use windows::Win32::System::Threading::{
    CreateProcessAsUserW,
    STARTUPINFOW,
    PROCESS_INFORMATION,
    CREATE_NEW_CONSOLE,
    STARTF_USESHOWWINDOW,
    GetCurrentProcess,
    OpenProcessToken
};
use windows::Win32::Security::{
    TOKEN_QUERY,
    PrivilegeCheck, PRIVILEGE_SET,
    LookupPrivilegeValueW, LUID_AND_ATTRIBUTES,
    TOKEN_PRIVILEGES,TOKEN_ADJUST_PRIVILEGES ,
    AdjustTokenPrivileges, SE_PRIVILEGE_ENABLED
};
use windows::Win32::Foundation::{HANDLE, BOOL, CloseHandle};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::core::{PCSTR, PWSTR, PSTR, PCWSTR};
use std::error::Error;
use std::ptr::null_mut;

pub struct SessionManager {
    server_handle: HANDLE,
    session_id: Option<u32>,
}

impl SessionManager {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        unsafe {
            println!("[SessionManager::new] [Debug] Attempting to open WTS server");
            let server_handle = WTSOpenServerA(PCSTR::null());
            if server_handle.is_invalid() {
                eprintln!("[SessionManager::new] [Error] Failed to open WTS server");
                return Err("Failed to open WTS server".into());
            }
            println!("[SessionManager::new] [Debug] Successfully opened WTS server: {:?}", server_handle);
            Ok(SessionManager {
                server_handle,
                session_id: None,
            })
        }
    }

    pub fn create_session(&mut self, token: HANDLE, username: &str) -> Result<u32, Box<dyn Error + Send + Sync>> {
        println!("[create_session] [Debug] Starting session creation for user: {}", username);
        println!("[create_session] [Debug] Token handle: {:?}", token);

        unsafe {
            let mut count: u32 = 0;
            let mut sessions: *mut WTS_SESSION_INFOA = null_mut();
            
            println!("[create_session] [Debug] Enumerating sessions");
            if !WTSEnumerateSessionsA(
                self.server_handle,
                0,
                1,
                &mut sessions,
                &mut count
            ).as_bool() {
                let error = Win32Error::get_last_error();
                eprintln!("[create_session] [Error] Failed to enumerate sessions: {}", error);
                return Err(format!("Failed to enumerate sessions: {}", error).into());
            }
            println!("[create_session] [Debug] Found {} sessions", count);

            let sessions_slice = std::slice::from_raw_parts(sessions, count as usize);
            
            // Look for existing disconnected session
            for session in sessions_slice {
                println!("[create_session] [Debug] Checking session ID: {}", session.SessionId);
                let mut buffer: PWSTR = PWSTR::null();
                let mut bytes_returned: u32 = 0;
                
                if WTSQuerySessionInformationA(
                    self.server_handle,
                    session.SessionId,
                    WTS_INFO_CLASS(5), // WTSUserName
                    &mut buffer as *mut PWSTR as *mut PSTR,
                    &mut bytes_returned
                ).as_bool() {
                    // let session_username = buffer.to_string().unwrap_or_default();
                    let session_username = if !buffer.is_null() {
                        let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8);
                        match c_str.to_str() {
                            Ok(s) => s.to_string(),
                            Err(e) => {
                                eprintln!("[create_session] [Error] Invalid UTF-8 in username: {}", e);
                                String::new()
                            }
                        }
                    } else {
                        String::new()
                    };

                    println!("[create_session] [Debug] Session username: {}", session_username);
                    
                    if session_username == username && session.State != WTSActive {
                        println!("[create_session] [Debug] Found existing disconnected session: {}", session.SessionId);
                        self.session_id = Some(session.SessionId);
                        return Ok(session.SessionId);
                    }
                } else {
                    eprintln!("[create_session] [Error] Failed to query session information for session: {}", session.SessionId);
                }
            }

            println!("[create_session] [Debug] No existing session found, creating new session");

            match enable_required_privileges() {
                Ok(()) => {
                    println!("Successfully enabled required privileges");
                    // Proceed with session creation
                },
                Err(e) => {
                    eprintln!("Failed to enable privileges: {}", e);
                    // Handle error appropriately
                }
            }
            

            if !check_required_privileges()? {
                return Err("Missing required privileges for creating user session".into());
            }

            let mut startup_info: STARTUPINFOW = STARTUPINFOW::default();
            startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
            startup_info.lpDesktop = PWSTR::null();
            startup_info.dwFlags = STARTF_USESHOWWINDOW;
            startup_info.wShowWindow = 1;
            
            let mut process_info: PROCESS_INFORMATION = PROCESS_INFORMATION::default();
            
            println!("[create_session] [Debug] Preparing to launch explorer.exe");
            let cmd = to_wide_string("C:\\Windows\\explorer.exe");
            let cmd_pwstr = PWSTR::from_raw(cmd.as_ptr() as *mut u16);
            
            let security_attributes = SECURITY_ATTRIBUTES {
                nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
                lpSecurityDescriptor: null_mut(),
                bInheritHandle: BOOL::from(false),
            };

            println!("[create_session] [Debug] Attempting to create process as user");
            if unsafe {
             CreateProcessAsUserW(
                token,
                PCWSTR::null(),
                cmd_pwstr,
                Some(&security_attributes),
                Some(&security_attributes),
                BOOL::from(false),
                CREATE_NEW_CONSOLE,
                Some(null_mut()),
                PCWSTR::null(),
                &startup_info,
                &mut process_info
               )
           }.as_bool() {
                println!("[create_session] [Debug] Process created successfully. Process ID: {}", process_info.dwProcessId);
                
                let mut bytes_returned: u32 = 0;
                let mut buffer: PWSTR = PWSTR::null();
                
                println!("[create_session] [Debug] Querying session ID for process");
                let result = WTSQuerySessionInformationA(
                    self.server_handle,
                    process_info.dwProcessId,
                    WTS_INFO_CLASS(9), // WTSSessionId
                    &mut buffer as *mut PWSTR as *mut PSTR,
                    &mut bytes_returned
                ).as_bool();

                println!("[create_session] [Debug] Cleaning up process handles");
                CloseHandle(process_info.hProcess);
                CloseHandle(process_info.hThread);

                if result {
                    let session_id_ptr = buffer.as_ptr() as *const u32;
                    let session_id = unsafe { *session_id_ptr };
                    println!("[create_session] [Debug] Successfully created session: {}", session_id);
                    self.session_id = Some(session_id);
                    return Ok(session_id);
                } else {
                    let last_error = Win32Error::get_last_error();
                    eprintln!("[create_session] [Error] Failed to query session ID for process: {}", {last_error});
                }
            } else {
                let last_error = Win32Error::get_last_error();
                eprintln!(
                    "[create_session] [Error] Failed to create process as user: {}",last_error
                );
    
            }
            
            eprintln!("[create_session] [Error] Failed to create new session");
            Err("Failed to create new session".into())
        }
    }

    pub fn get_current_session_id(&self) -> Option<u32> {
        println!("[get_current_session_id] [Debug] Current session ID: {:?}", self.session_id);
        self.session_id
    }
}

impl Drop for SessionManager {
    fn drop(&mut self) {
        unsafe {
            println!("[SessionManager::drop] [Debug] Closing WTS server handle");
            WTSCloseServer(self.server_handle);
        }
    }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    println!("[to_wide_string] [Debug] Converting string to wide string: {}", s);
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn check_required_privileges() -> Result<bool, Box<dyn Error + Send + Sync>> {
    let required_privileges = [
        "SeAssignPrimaryTokenPrivilege",
        "SeIncreaseQuotaPrivilege",
        "SeSecurityPrivilege",
    ];

    unsafe {
        // Get current process handle
        let process_handle = GetCurrentProcess();
        
        // Get process token
        let mut token_handle = HANDLE::default();
        if !OpenProcessToken(
            process_handle,
            TOKEN_QUERY,
            &mut token_handle
        ).as_bool() {
            return Err("Failed to open process token".into());
        }
        
        // RAII guard for token handle
        let _token_guard = HandleGuard(token_handle);

        // Check each required privilege
        for privilege_name in required_privileges.iter() {
            let wide_name = to_wide_string(privilege_name);
            let mut luid = windows::Win32::Foundation::LUID::default();
            
            // Look up the LUID for the privilege
            if !LookupPrivilegeValueW(
                PCWSTR::null(),
                PCWSTR::from_raw(wide_name.as_ptr()),
                &mut luid
            ).as_bool() {
                return Err(format!("Failed to look up privilege value for {}", privilege_name).into());
            }

            // Set up the privilege set for checking
            let mut privilege_set = PRIVILEGE_SET {
                PrivilegeCount: 1,
                Control: 1, // PRIVILEGE_SET_ALL_NECESSARY
                Privilege: [LUID_AND_ATTRIBUTES {
                    Luid: luid,
                    Attributes: windows::Win32::Security::SE_PRIVILEGE_ENABLED,
                }],
            };

            let mut privilege_result: i32 = 0;
            
            // Check if the privilege is enabled
            if !PrivilegeCheck(
                token_handle,
                &mut privilege_set,
                &mut privilege_result
            ).as_bool() {
                return Err(format!("Failed to check privilege status for {}", privilege_name).into());
            }

            if privilege_result == 0 {
                println!("[check_required_privileges] [Warning] Missing privilege: {}", privilege_name);
                return Ok(false);
            }
        }

        println!("[check_required_privileges] [Debug] All required privileges are present");
        Ok(true)
    }
}

fn enable_required_privileges() -> Result<(), Box<dyn Error + Send + Sync>> {
    let required_privileges = [
        "SeAssignPrimaryTokenPrivilege",
        "SeIncreaseQuotaPrivilege",
        "SeSecurityPrivilege",
    ];

    unsafe {
        // Get current process handle
        let process_handle = GetCurrentProcess();
        
        // Get process token with adjust privileges rights
        let mut token_handle = HANDLE::default();
        if !OpenProcessToken(
            process_handle,
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle
        ).as_bool() {
            return Err("Failed to open process token".into());
        }
        
        let _token_guard = HandleGuard(token_handle);

        // Enable each required privilege
        for privilege_name in required_privileges.iter() {
            let wide_name = to_wide_string(privilege_name);
            let mut luid = windows::Win32::Foundation::LUID::default();
            
            println!("[enable_required_privileges] [Debug] Enabling privilege: {}", privilege_name);
            
            if !LookupPrivilegeValueW(
                PCWSTR::null(),
                PCWSTR::from_raw(wide_name.as_ptr()),
                &mut luid
            ).as_bool() {
                return Err(format!("Failed to look up privilege value for {}", privilege_name).into());
            }

            let mut new_privileges = TOKEN_PRIVILEGES {
                PrivilegeCount: 1,
                Privileges: [LUID_AND_ATTRIBUTES {
                    Luid: luid,
                    Attributes: SE_PRIVILEGE_ENABLED,
                }],
            };

            // Adjust token privileges
            if !AdjustTokenPrivileges(
                token_handle,
                false,
                Some(&mut new_privileges),
                0,
                None,
                None,
            ).as_bool() {
                return Err(format!("Failed to adjust token privileges for {}", privilege_name).into());
            }
        }

        println!("[enable_required_privileges] [Debug] Successfully enabled all required privileges");
        Ok(())
    }
}

// RAII wrapper for Windows handles
struct HandleGuard(HANDLE);