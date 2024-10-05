use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::io::Result;
use tokio::net::tcp::OwnedWriteHalf;


pub async fn capture_and_send_input(mut stream: OwnedWriteHalf) -> Result<()> {
    loop {
        // Capture left mouse button state
        let lmb_state = unsafe { GetAsyncKeyState(VK_LBUTTON) };

        // Capture right mouse button state
        let rmb_state = unsafe { GetAsyncKeyState(VK_RBUTTON) };

        // Capture keyboard state (e.g., VK_A for 'A' key)
        let a_key_state = unsafe { GetAsyncKeyState(0x41) }; // 'A' key

        // Example: sending mouse states and keyboard key states
        let input_state = format!("{},{},{}", lmb_state, rmb_state, a_key_state);

        // Send the input data to the server
        stream.write_all(input_state.as_bytes()).await?;
    }
}
