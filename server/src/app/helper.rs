// use windows::{
//     Win32::NetworkManagement::IpHelper::*,
//     Win32::Networking::WinSock::*,
//     core::*,
//     Win32::Foundation::*,
// };
// use std::net::IpAddr;
// use std::mem;

// pub async fn get_local_ip() -> Result<IpAddr> {
//     unsafe {
//         let mut size = 0u32;
//         GetAdaptersAddresses(
//             AF_UNSPEC.0 as u32,
//             0,
//             std::ptr::null_mut(),
//             std::ptr::null_mut(),
//             &mut size,
//         );

//         let mut buffer = vec![0u8; size as usize];
//         let mut addresses_ptr = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

//         let result = GetAdaptersAddresses(
//             AF_UNSPEC.0 as u32,
//             0,
//             std::ptr::null_mut(),
//             addresses_ptr,
//             &mut size,
//         );

//         if result != ERROR_SUCCESS.0 {
//             return Err(windows::core::Error::from_win32());
//         }

//         while !addresses_ptr.is_null() {
//             let addresses = &*addresses_ptr;
            
//             if addresses.OperStatus == IfOperStatusUp {
//                 let mut unicast = addresses.FirstUnicastAddress;
//                 while !unicast.is_null() {
//                     let unicast_ref = &*unicast;
//                     let socket_addr = unicast_ref.Address.lpSockaddr;
//                     if !socket_addr.is_null() {
//                         let family = (*socket_addr).sa_family;
//                         if family == AF_INET.0 {
//                             let sock_addr_in: *const SOCKADDR_IN = mem::transmute(socket_addr);
//                             let ip_addr = IpAddr::V4(std::net::Ipv4Addr::from(
//                                 (*sock_addr_in).sin_addr.S_un.S_addr.to_ne_bytes()
//                             ));
//                             return Ok(ip_addr);
//                         } else if family == AF_INET6.0 {
//                             let sock_addr_in6: *const SOCKADDR_IN6 = mem::transmute(socket_addr);
//                             let ip_addr = IpAddr::V6(std::net::Ipv6Addr::from(
//                                 (*sock_addr_in6).sin6_addr.u.Byte
//                             ));
//                             return Ok(ip_addr);
//                         }
//                     }
//                     unicast = unicast_ref.Next;
//                 }
//             }
//             addresses_ptr = addresses.Next;
//         }

//         Err(windows::core::Error::new(ERROR_NOT_FOUND, HSTRING::from("No IP address found")))
//     }
// }