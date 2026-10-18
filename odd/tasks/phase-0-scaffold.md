# Phase 0 — Project scaffold

## Objective

Bootstrap the FileConverter repository so Phase 1 (pure core, TDD) can start immediately.

## Problem / Why

The project is empty. We need a Tauri 2 + React/TS desktop shell, a Cargo workspace with the
hexagonal crate layout (`core`, `runtime`, later `engine-*`), and CI that keeps fmt/clippy/tests green
on Windows, macOS and Linux.

## Scope

- In: Tauri 2 app (`apps/desktop`, React + TS + Vite, pnpm), root Cargo workspace, `crates/core` and
  `crates/runtime` skeletons with a smoke test, CI workflow, `.gitignore`, README.
- Out: conversion logic (Phase 1+), the LibreOffice packaging spike (separate feature, needs a
  ~350 MB download approved by the user).

## Constraints

- 100% offline at runtime: no network plugins, no auto-updater.
- `core` and `runtime` must not depend on Tauri.
- Conventional commits, no AI attribution.

## Decisions

- Single installer with every engine bundled, LibreOffice included (user decision, 2026-10-18).
- Toolchain: Rust 1.99.1 stable MSVC, Node 24, pnpm 12.

## Tasks

- [x] T1 — Scaffold Tauri 2 + React/TS app in `apps/desktop` (route: inline, generator output)
- [x] T2 — Root Cargo workspace + `crates/core` + `crates/runtime` skeletons (route: inline, trivial stubs)
- [x] T3 — CI workflow: fmt, clippy, tests, frontend build on 3 OSes (route: inline, one mechanical file)
- [x] T4 — Verify locally: `cargo fmt --check`, `cargo clippy`, `cargo test`, `pnpm build`
- [x] T5 — Work-unit commit (`f0f67a6`)

## Acceptance criteria

- `cargo test --workspace` passes, including a smoke test in `core`.
- `cargo clippy --workspace --all-targets -- -D warnings` is clean.
- `pnpm --dir apps/desktop build` succeeds.

## Progress / Evidence

Observed locally on Windows 11 (2026-10-18):

- `pnpm --dir apps/desktop build`: OK (vite 8.3.4).
- `cargo fmt --all --check`: OK.
- `cargo clippy --workspace --all-targets -- -D warnings`: OK, no warnings.
- `cargo test --workspace`: 2 passed (`converter-core`, `converter-runtime`), 0 failed.
- `target/debug/file-converter.exe` launched and stayed alive for 8 s, then was stopped.
- Not verified: CI on GitHub (no remote yet), macOS and Linux builds.

Notes:

- Test-first exception: scaffolding has no behavior to drive with RED/GREEN; checks are structural.
- Crate names are `converter-core` / `converter-runtime` because a package named `core` shadows Rust's `core`.
- `tauri::generate_context!` needs `apps/desktop/dist` at compile time, so CI builds the frontend first.
- CSP blocks every network origin; `devCsp` only adds the Vite HMR websocket.

## Review

- Native review (4 lenses, risk high, user granted) on `f0f67a6..2f6ce4a`: approved and acknowledged,
  lineage `review-2711a49137d1d0ab`. Risk and resilience lenses: no findings.
- Advisory, non-blocking follow-ups for the start of Phase 1:
  - Remove the Tauri template `greet` demo (`App.tsx`, `lib.rs`) and set the `index.html` title.
  - Replace the tautological version smoke tests; Phase 1 domain tests supersede them.
  - Single source of truth for the app version (workspace, `package.json`, `tauri.conf.json`).
  - Type-check `vite.config.ts` (`tsc -b` or include `tsconfig.node.json`).
  - Add `--locked` to the README check commands to match CI.
  - Deduplicate the root and `apps/desktop` `.gitignore` rules.

## Next step

Phase 0 is complete. Next: LibreOffice packaging spike (needs download approval), then Phase 1.
