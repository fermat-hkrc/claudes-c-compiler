# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_c_lui, English, tier standard)
> Incremental: previous campaigns remain in COVERAGE.md. This campaign added compressed.rs / encode_c_lui.
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_c_lui NOT LINKED in C++ pbt binaries (Rust `cargo test --lib` is not those binaries).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_c_lui (compressed.rs:6) |
| Properties | 9 (7 passing, 2 failing) |
| KAT | 6 passing |
| Regression witnesses | 2 failing (bugs) |
| Sweep | 1 round (signed-vs-uimm20 metamorphic, passing) |
| Bugs | 2 (extra operand ignored; oob imm truncated) |

## Module Breakdown (this campaign)

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
| encode_c_lui | 1 | 1 | 0 | 100% of HARD-scoped symbol |

Other compressed.rs functions were indexed and skipped (HARD: test only encode_c_lui).
