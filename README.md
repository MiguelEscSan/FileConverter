# FileConverter

Offline desktop file converter. Every conversion runs locally: no file ever leaves the machine.

## Stack

- Shell: Tauri 2 (`apps/desktop`), frontend in React + TypeScript + Vite.
- Core: Rust workspace with a hexagonal layout.
  - `crates/core` — conversion domain and ports (formats, capabilities, `Converter`, route planner). No IO.
  - `crates/runtime` — job queue, per-engine concurrency, cancellation, temporary workspaces. No Tauri.
  - `crates/engine-*` — one adapter per engine (images, Pandoc, LibreOffice, PDFium, later FFmpeg).
- Distribution: a single installer that bundles every engine, LibreOffice included.

## Requirements

- Rust stable (MSVC toolchain on Windows), Node 24, pnpm 12.
- Platform prerequisites from <https://tauri.app/start/prerequisites/>.

## Development

```bash
pnpm --dir apps/desktop install
pnpm --dir apps/desktop tauri dev
```

## Checks

```bash
pnpm --dir apps/desktop build
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
