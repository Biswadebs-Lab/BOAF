//! Target liveness / "is it still up" detection.
//!
//! Used after a failed transmission to tell a genuine target
//! crash/down event apart from a one-off transient network error
//! (a dropped packet, a momentary RST, a timed-out single attempt)
//! before reporting a crash boundary.

use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::time::Duration;

/// Attempts a single quick TCP connect to decide whether the target
/// is currently accepting connections at all. Connection-level only
/// (no data is sent) so it doesn't itself perturb the target.
pub fn is_alive(socket_addrs: &[SocketAddr], connect_timeout: Duration) -> bool {
    match socket_addrs.first() {
        Some(addr) => TcpStream::connect_timeout(addr, connect_timeout).is_ok(),
        None => false,
    }
}

/// Re-checks the target several times with a short delay between
/// attempts. Returns `true` only if *every* attempt fails, i.e. the
/// target is confirmed down rather than having just dropped one
/// connection. Returns `false` (not confirmed down) as soon as any
/// attempt succeeds.
pub fn confirm_down(
    socket_addrs: &[SocketAddr],
    attempts: usize,
    delay_between_attempts: Duration,
    connect_timeout: Duration,
) -> bool {
    let attempts = attempts.max(1);
    for attempt in 0..attempts {
        if is_alive(socket_addrs, connect_timeout) {
            return false;
        }
        if attempt + 1 < attempts {
            thread::sleep(delay_between_attempts);
        }
    }
    true
}
