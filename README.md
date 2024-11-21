# Fast and Memory-Safe RDP (Remote Desktop Protocol)

## Overview

This project is a fast and memory-safe Remote Desktop Protocol (RDP) client and server implementation built using Rust. It leverages the Windows API for authentication and screen capture, offering an efficient and secure way to remotely access desktops with minimal latency and high performance. The project demonstrates the capabilities of Rust for building robust systems applications that prioritize speed, safety, and efficiency.

## Features

- **Secure Authentication**: Utilizes `LogonUserW` from the Windows API for secure credential management.
- **Real-time Screen Capture**: Captures desktop screen using `BitBlt` and `CreateCompatibleBitmap`, ensuring efficient screen rendering.
- **Data Compression**: Compresses image data with `lz4_flex`, enabling faster transmission with minimal bandwidth usage.
- **Low Latency Streaming**: Optimized for quick response times, providing near real-time desktop interaction.
- **Memory Safety**: Built using Rust to ensure thread safety, prevent memory leaks, and eliminate buffer overflows.

## Project Structure

- **Server**: Captures the server's screen, compresses the data, and streams it to the client via TCP.
- **Client**: Receives compressed screen data, decompresses it, and renders it using `minifb`.

## Technologies

- **Rust**: Core language used for building both client and server.
- **Windows API**: Used for screen capture (`BitBlt`, `GetSystemMetrics`, etc.) and authentication (`LogonUserW`).
- **lz4_flex**: Compression library for fast and efficient data compression and decompression.
- **minifb**: Minimal framebuffer window to render the received remote desktop screen.
- **Tokio**: Asynchronous runtime for managing TCP connections and efficient data streaming.

## Installation

1. **Clone the repository**:

    ```bash
    git clone git@github.com:katalystaditya/xRDP.git
    cd xrdp
    ```

2. **Install Rust**:

    Ensure you have Rust installed. You can download it [here](https://www.rust-lang.org/tools/install).
    

3. **Install dependencies**:

    ```bash
    1.client
    cd client 
    npm install

    2.server
    cargo build
    ```

4. **Run the server**:

    ```bash
    cd server
    cargo run 
    ```

5. **Run the client**:

    ```bash
    cd client
    npm run tauri dev
    ```

## Usage

1. Start the **server** on the machine you want to control remotely.
2. Start the **client** on your local machine, and connect to the server using the server's IP address and credentials.
3. Use the client window to view and interact with the remote desktop.

## Future Improvements

- **Cross-Platform Support**: Extend to non-Windows environments for broader usability.
- **Enhanced Compression**: Explore more advanced compression algorithms for higher efficiency.
- **Encryption**: Add end-to-end encryption for enhanced data security.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For more information or support, contact [aditya.kumar@katalystpartners.com](mailto:aditya.kumar@katalystpartners.com).