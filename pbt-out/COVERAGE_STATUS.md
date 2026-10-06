# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_ldr_str, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). cargo test --lib encode_ldr_str executed the real symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_ldr_str (load_store.rs:33) |
| Properties | 16 (8 passing, 8 failing) |
| Bugs | 8 |
| Sweep | 1/1 (SIMD, alt-spellings, literal reloc) |
