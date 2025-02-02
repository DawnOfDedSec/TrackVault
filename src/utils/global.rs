use base64::{engine::general_purpose, Engine as _};
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

pub fn to_base64(plain_text: &str) -> String {
    general_purpose::STANDARD.encode(plain_text.as_bytes())
}

// Function to measure the latency of a given domain and port
pub fn get_latency(domain: &str, port: u16) -> io::Result<Duration> {
    // Resolve the domain to an IP address
    let addr: SocketAddr = format!("{}:{}", domain, port).parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to parse address: {}", e),
        )
    })?;

    // Measure the time taken to establish a connection
    let start = Instant::now();
    match TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
        Ok(_) => {
            let duration = start.elapsed();
            Ok(duration)
        }
        Err(e) => Err(e),
    }
}
