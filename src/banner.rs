//! Console banner / tool signature. No behavior beyond printing.

const RED: &str = "\x1b[38;2;178;0;0m";
const RED_BOLD: &str = "\x1b[1;38;2;220;20;20m";
const WHITE: &str = "\x1b[38;2;235;235;235m";
const GRAY: &str = "\x1b[38;2;150;150;150m";
const RESET: &str = "\x1b[0m";

/// Prints a stylized ASCII banner and professional signature to the console interface.
pub fn print_banner() {
    let logo = [
        "██████╗  ██████╗  █████╗ ███████╗",
        "██╔══██╗██╔═══██╗██╔══██╗██╔════╝",
        "██████╔╝██║   ██║███████║█████╗  ",
        "██╔══██╗██║   ██║██╔══██║██╔══╝  ",
        "██████╔╝╚██████╔╝██║  ██║██║     ",
        "╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝     ",
    ];

    // Right-hand column, line by line (plain text; colored selectively below)
    let right: [&str; 7] = [
        "Buffer-Overflow",
        "Automated Framework v1.0",
        "",
        "Made by: Biswadeb Mukherjee",
        "Role:    Security Engineer & Researcher",
        "Web:     official-biswadeb941.in",
        "Email:   admin@official-biswadeb941.in",
    ];

    let logo_rows = logo.len();
    let left_w = logo.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    println!();
    let rows = logo_rows.max(right.len());
    for i in 0..rows {
        let l = logo.get(i).copied().unwrap_or("");
        let r = right.get(i).copied().unwrap_or("");

        let right_colored = if r.ends_with("v1.0") {
            let prefix = &r[..r.len() - 4];
            format!("{WHITE}{prefix}{RED_BOLD}v1.0{RESET}")
        } else if let Some(stripped) = r.strip_prefix("Made by: ") {
            format!("{GRAY}Made by: {RESET}{WHITE}{stripped}{RESET}")
        } else if let Some(stripped) = r.strip_prefix("Role:    ") {
            format!("{GRAY}Role:    {RESET}{WHITE}{stripped}{RESET}")
        } else if let Some(stripped) = r.strip_prefix("Web:     ") {
            format!("{GRAY}Web:     {RESET}{WHITE}{stripped}{RESET}")
        } else if let Some(stripped) = r.strip_prefix("Email:   ") {
            format!("{GRAY}Email:   {RESET}{WHITE}{stripped}{RESET}")
        } else {
            format!("{WHITE}{r}{RESET}")
        };

        println!(
            "  {RED_BOLD}{:<lw$}{RESET}  {GRAY}│{RESET} {right_colored}",
            l,
            lw = left_w
        );
    }

    let rule_w = left_w + 46;
    println!("{RED}{}{RESET}", "─".repeat(rule_w));
    println!();
}