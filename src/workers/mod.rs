//! Worker management for BOAF's progressive-fuzzing test-execution path.
//!
//! Scope is intentionally limited to parallelising the size-sweep loop
//! against a single authorised target (AGENTS.md: "Test execution and
//! worker management" under permitted fuzzing functionality). It does
//! not touch payload/exploit construction.

mod pool;

pub use pool::WorkerPool;
