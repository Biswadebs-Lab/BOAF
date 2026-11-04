use clap::{CommandFactory, Parser};
use std::net::ToSocketAddrs;
use std::sync::Arc;

mod banner;
mod bof;
mod liveness;
mod workers;

use banner::print_banner;
use bof::ProtocolMode;
use workers::WorkerPool;

#[derive(Parser, Debug)]
#[command(name = "boaf")]
#[command(author = "Biswadeb Mukherjee <official-biswadeb941.in>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "High-Performance Network Fuzzer & Buffer-Overflow Automated Framework", long_about = None)]
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

    /// Number of concurrent worker threads used during fuzzing
    #[arg(short = 'w', long, default_value_t = 4)]
    workers: usize,

    /// Reconnect attempts used to confirm the target is actually down
    /// after a failed send, before reporting a crash boundary
    #[arg(long, default_value_t = 3)]
    down_check_attempts: usize,

    /// Delay between down-check reconnect attempts, in milliseconds
    #[arg(long, default_value_t = 500)]
    down_check_delay_ms: u64,

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
    let (target_host, target_port) = bof::parse_target_and_port(&target_input);
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
        println!("[*] Worker threads: {}", args.workers);

        let pool = WorkerPool::new(args.workers);
        let result = pool.run(
            Arc::new(socket_addrs.clone()),
            Arc::new(target_host.clone()),
            target_port,
            args.mode,
            args.start_size,
            args.max_size,
            args.step,
            args.allow_self_signed,
            args.down_check_attempts,
            std::time::Duration::from_millis(args.down_check_delay_ms),
        );

        match result {
            Some((length, reason, true)) => {
                println!("\n[!] TARGET CONFIRMED DOWN");
                println!("[!] Smallest observed failing transmission length: {} bytes", length);
                println!("[!] Reason reported by network interface layer: {}", reason);
            }
            Some((length, reason, false)) => {
                println!("\n[?] Transient failure observed; target is still responding to connections");
                println!("[?] Smallest observed failing transmission length: {} bytes", length);
                println!("[?] Reason reported by network interface layer: {}", reason);
                println!("[?] Increase --down-check-attempts/--down-check-delay-ms if this needs more confirmation.");
            }
            None => {
                println!("[+] No instability detected up to {} bytes.", args.max_size);
            }
        }
    } else {
        bof::run_static_injector(
            &socket_addrs,
            &target_host,
            target_port,
            args.mode,
            args.allow_self_signed,
        )?;
    }

    Ok(())
}
