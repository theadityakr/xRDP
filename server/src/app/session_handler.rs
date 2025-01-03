use windows::Win32::System::RemoteDesktop::{
    WTSOpenServerA, WTSCloseServer, WTSQuerySessionInformationA,
    WTSDisconnectSession, WTSLogoffSession, WTS_CURRENT_SESSION,
    WTSUserName, WTSClientProtocolType
};
use rand::Rng;
use windows::Win32::Foundation::{HANDLE, CloseHandle};
use windows::core::{PWSTR, PSTR, PCSTR};
use std::error::Error;


pub async fn create_or_get_session(token: HANDLE) -> Result<u32, Box<dyn Error + Send + Sync>>{
    unsafe {
        let server_handle = WTSOpenServerA(PCSTR::null());
        if server_handle.is_invalid() {
            return Err("[create_or_get_session] [Error] : Failed to open server".into());
        }

        let mut bytes_returned: u32 = 0;
        let mut buffer: PWSTR = PWSTR::null();
        
        let result = WTSQuerySessionInformationA(
            server_handle,
            WTS_CURRENT_SESSION,
            WTSClientProtocolType,
            &mut buffer as *mut PWSTR as *mut PSTR,
            &mut bytes_returned,
        );

        if !result.as_bool() {
            WTSCloseServer(server_handle);
            return Err("[create_or_get_session] [Error] : Failed to query session information".into());
        }

        let mut rng = rand::thread_rng();
        let session_id =  rng.gen::<u32>();

        WTSCloseServer(server_handle);

        Ok(session_id)
    }
}

pub async fn cleanup_session(token: HANDLE, session_id: u32) {
    unsafe {
        let server_handle = WTSOpenServerA(PCSTR::null());
        if !server_handle.is_invalid() {
            WTSDisconnectSession(server_handle, session_id, false);
            WTSLogoffSession(server_handle, session_id, false);
            WTSCloseServer(server_handle);
        }
        CloseHandle(token);
    }
}
