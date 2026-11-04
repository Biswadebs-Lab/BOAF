//! Concurrent executor for the progressive fuzzing size-sweep.
//!
//! Splits the `[start_size, max_size]` sweep across N worker threads
//! using a shared atomic cursor (work-stealing), so iterations run in
//! parallel instead of one request at a time. A failed send doesn't
//! immediately count as a crash boundary: it's handed to `liveness`
//! for a few reconnect attempts first, so a transient blip (one
//! dropped connection) can't halt the sweep as if the target were
//! actually down. Workers stop claiming new lengths once any worker
//! confirms the target is down at a smaller length, so the result
//! still reflects the smallest confirmed failure rather than racing
//! past it.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::bof::{assemble_packet, connect_and_transmit, ProtocolMode};
use crate::liveness;

struct TestOutcome {
    length: usize,
    failure_reason: Option<String>,
    confirmed_down: bool,
}

/// Runs the progressive fuzzing sweep across a fixed-size thread pool.
pub struct WorkerPool {
    worker_count: usize,
}

impl WorkerPool {
    pub fn new(worker_count: usize) -> Self {
        Self {
            worker_count: worker_count.max(1),
        }
    }

    /// Executes the sweep and returns `(length, reason, confirmed_down)`
    /// for the smallest failing length observed, or `None` if every
    /// length in range passed. `confirmed_down` is `true` only when
    /// the target failed to answer across every down-check attempt;
    /// `false` means the failure was transient (the target answered
    /// again on a later liveness check) but is still reported so it
    /// isn't silently dropped.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &self,
        socket_addrs: Arc<Vec<SocketAddr>>,
        host: Arc<String>,
        port: u16,
        mode: ProtocolMode,
        start_size: usize,
        max_size: usize,
        step: usize,
        allow_self_signed: bool,
        down_check_attempts: usize,
        down_check_delay: Duration,
    ) -> Option<(usize, String, bool)> {
        let step = step.max(1);
        let next_len = Arc::new(AtomicUsize::new(start_size));
        let stop_at = Arc::new(AtomicUsize::new(usize::MAX));
        let (tx, rx) = mpsc::channel::<TestOutcome>();

        let handles: Vec<_> = (0..self.worker_count)
            .map(|_| {
                let next_len = Arc::clone(&next_len);
                let stop_at = Arc::clone(&stop_at);
                let socket_addrs = Arc::clone(&socket_addrs);
                let host = Arc::clone(&host);
                let tx = tx.clone();

                thread::spawn(move || {
                    loop {
                        let length = next_len.fetch_add(step, Ordering::SeqCst);
                        if length > max_size || length >= stop_at.load(Ordering::SeqCst) {
                            break;
                        }

                        let buffer = vec![b'A'; length];
                        let packet = assemble_packet(&buffer, mode, &host);

                        let outcome = match connect_and_transmit(
                            &socket_addrs,
                            &host,
                            port,
                            &packet,
                            allow_self_signed,
                        ) {
                            Ok(_) => TestOutcome {
                                length,
                                failure_reason: None,
                                confirmed_down: false,
                            },
                            Err(e) => {
                                // Don't trust a single failed send: reconnect a
                                // few times before concluding the target is down.
                                let down = liveness::confirm_down(
                                    &socket_addrs,
                                    down_check_attempts,
                                    down_check_delay,
                                    Duration::from_secs(2),
                                );
                                if down {
                                    stop_at.fetch_min(length, Ordering::SeqCst);
                                }
                                TestOutcome {
                                    length,
                                    failure_reason: Some(e.to_string()),
                                    confirmed_down: down,
                                }
                            }
                        };

                        if tx.send(outcome).is_err() {
                            break;
                        }

                        thread::sleep(Duration::from_millis(50));
                    }
                })
            })
            .collect();
        drop(tx);

        let mut earliest_confirmed: Option<(usize, String)> = None;
        let mut earliest_transient: Option<(usize, String)> = None;

        for outcome in rx {
            println!("[...] Testing stability limit footprint: {} bytes", outcome.length);
            if let Some(reason) = outcome.failure_reason {
                if outcome.confirmed_down {
                    println!("[!] Target DOWN confirmed at {} bytes", outcome.length);
                    let is_earlier = earliest_confirmed
                        .as_ref()
                        .map_or(true, |(len, _)| outcome.length < *len);
                    if is_earlier {
                        earliest_confirmed = Some((outcome.length, reason));
                    }
                } else {
                    println!(
                        "[?] Transient error at {} bytes (target still answering): {}",
                        outcome.length, reason
                    );
                    let is_earlier = earliest_transient
                        .as_ref()
                        .map_or(true, |(len, _)| outcome.length < *len);
                    if is_earlier {
                        earliest_transient = Some((outcome.length, reason));
                    }
                }
            }
        }

        for handle in handles {
            let _ = handle.join();
        }

        // A confirmed-down result always takes priority; otherwise surface
        // the earliest transient failure so it isn't lost, flagged as such.
        match earliest_confirmed {
            Some((len, reason)) => Some((len, reason, true)),
            None => earliest_transient.map(|(len, reason)| (len, reason, false)),
        }
    }
}
