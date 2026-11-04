//! Console banner / tool signature. No behavior beyond printing.

/// Prints a stylized ASCII banner and professional signature to the console interface.
pub fn print_banner() {
    let banner = r#"
========================================================================
    ██████╗  ██████╗  █████╗ ███████╗
    ██╔══██╗██╔═══██╗██╔══██╗██╔════╝
    ██████╔╝██║   ██║███████║█████╗
    ██╔══██╗██║   ██║██╔══██║██╔══╝
    ██████╔╝╚██████╔╝██║  ██║██║
    ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝

  [ Buffer-Overflow Automated Framework ]

  Made by: Biswadeb Mukherjee
  Role:    Security Engineer & Researcher
  Web:     official-biswadeb941.in
========================================================================
"#;
    println!("{}", banner);
}
