use std::error::Error;
use windows::Win32::NetworkManagement::WNet::{WNetAddConnection2W, NETRESOURCEW, RESOURCE_CONNECTED, RESOURCETYPE_DISK};
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{NO_ERROR};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientDriveInfo {
    pub drive_letter: String, 
    pub mapped_drive_letter: String, 
}

pub async fn receive_client_drive_info(read_half: &mut ReadHalf<TcpStream>) -> Result<ClientDriveInfo, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 1024];
    let n = read_half.read(&mut buf).await?;

    let client_drive_info: ClientDriveInfo = serde_json::from_slice(&buf[..n])?;
    println!("Received drive info: {:?} -> {:?}", client_drive_info.drive_letter, client_drive_info.mapped_drive_letter);

    Ok(client_drive_info)
}
pub async fn redirect_client_drives(client_drive_info: &ClientDriveInfo,addr :SocketAddr) -> windows::core::Result<()> {

    let remote_drive = format!(r"\\{}\{}", addr.ip(), client_drive_info.drive_letter);
    let local_name = format!("{}:", client_drive_info.mapped_drive_letter); 

    println!("Attempting to map remote drive: {}", remote_drive);

    let remote_name = wide_string(&remote_drive);
    let local_name = wide_string(&local_name);

    let net_resource = NETRESOURCEW {
        dwScope: RESOURCE_CONNECTED,
        dwType: RESOURCETYPE_DISK,
        dwDisplayType: 0,
        dwUsage: 0,
        lpLocalName: PWSTR(local_name.as_ptr() as *mut u16),
        lpRemoteName: PWSTR(remote_name.as_ptr() as *mut u16),
        lpComment: PWSTR::null(),
        lpProvider: PWSTR::null(),
    };

    unsafe {
        let result = WNetAddConnection2W(
            &net_resource,
            PCWSTR::null(), // No password
            PCWSTR::null(), // Current user
            0
        );
        if result != NO_ERROR.0 {
            let error_code = windows::Win32::Foundation::GetLastError();
            eprintln!("Failed with error code: {}", error_code.0);
            return Err(windows::core::Error::from_win32());
        }
    }

    Ok(())
}

fn wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}
