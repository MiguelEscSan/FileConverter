//! Conversion domain and ports.
//!
//! This crate must stay free of IO and of any UI framework: it only models
//! formats, converter capabilities and route planning. Engines live in
//! `engine-*` adapter crates and the job orchestration lives in `converter-runtime`.

/// Version of the conversion domain crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_crate_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
