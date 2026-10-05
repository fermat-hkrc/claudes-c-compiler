# PBT Campaign: src/common (ccc C compiler — common utilities)

Spec: (none — whole-scope campaign; change surface `commit:HEAD` touches no source function)

## Scan findings
- **Test layout:** inline Rust `#[cfg(test)] mod tests` blocks inside source files (convention visible in `src/common/encoding.rs`, `src/common/long_double.rs`, ~30 other files). No `tests/` integration dir. Modules are `pub(crate)`, so integration tests cannot reach them — inline test blocks are the only project-conventional placement. Historical `pbt-out/` excluded from layout search.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib` (from `pbt-out/rounds/01_common/run/`) → 493 passed, 0 failed, 6 ignored (pre-existing baseline green).
- **PBT framework acquisition:** repo had no PBT dependency; `cargo add --dev proptest` (language package manager, user scope) succeeded → `proptest = "1.11.0"` in Cargo.toml `[dev-dependencies]`; `cargo test --lib --no-run` compiles clean. Build contract honored: rebuild target swapped to the test target of the same command (`cargo check --lib` → `cargo test --lib`).
- **Harness placement:** rung 1 — extend the repo's own inline test convention by appending a `#[cfg(test)] mod pbt_tests` block to each target source file (`src/common/{encoding,const_arith,const_eval,long_double,types}.rs`). No build-file surgery needed; cargo discovers inline tests.
- **Candidate modules:** `common::encoding` (PUA encode/decode round-trip), `common::const_arith` (C-semantics constant binops, truncate_and_extend_bits, negate/bitnot), `common::const_eval` (builtin bitop folding, promotions), `common::long_double` (f64↔f128↔x87 conversions, software arithmetic), `common::types` (align_up).
- **Skipped modules:** `common::error.rs` (diagnostics formatting/UI-driven, crash-only value), `common::source.rs` (file I/O + position bookkeeping, integration-bound), `common::symbol_table.rs` (thin map wrapper over tested hashing), `common::temp_files.rs` (filesystem side effects), `common::asm_constraints.rs` (table lookup, exercised via backend tests), `common::type_builder.rs` (builder over types.rs, covered indirectly), `common::fx_hash.rs` (vendored rustc algorithm, FxHasher semantics unchanged from upstream — noted as vendored).

## Module: common (encoding, const_arith, const_eval, long_double, types)
- [x] Scan: identify targets (FUNCTION_INDEX.md: 382 functions; primary targets above)
- [x] Plan: formalize properties (PROPERTIES.md — 14 entries)
- [x] Test: write and run (proptest, 1024 cases/property)
- [x] Review: triage results (see REPORT.md)
