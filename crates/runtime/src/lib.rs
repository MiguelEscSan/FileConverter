//! Job orchestration for conversions.
//!
//! Runs `converter_core` converters off the UI thread: job queue, per-engine
//! concurrency limits, cancellation, timeouts and per-job temporary workspaces.
//! Like the core, it must not depend on Tauri; binary paths are injected by the
//! desktop composition root.

/// Version of the conversion domain this runtime was built against.
pub fn core_version() -> &'static str {
    converter_core::VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_against_core() {
        assert_eq!(core_version(), converter_core::VERSION);
    }
}
