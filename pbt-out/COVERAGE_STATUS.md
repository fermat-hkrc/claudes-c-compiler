# Coverage status

**Campaign:** encode_neon_three_diff
**Tier:** standard
**Coverage evidence:** file-level (symbol presence)

coverage_gaps after the first full cargo test run: no .gcda/.profraw (Rust lib tests are not the C++ gcov reporter). The tool listed unrelated C++ binaries and claimed encode_neon_three_diff NOT LINKED. That is a reporter mismatch: `cargo test --lib encode_neon_three_diff_pbt` compiled and executed the production symbol (KAT + 1000-case properties).

Manual arm audit of encode_neon_three_diff (neon.rs:89-114):
- arity < 3 — driven (neg_arity, passing)
- get_neon_reg dest/Vn/Vm — driven
- match arr_n 8b/16b/4h/8h/2s/4s — driven (LONG differential, 1000 cases)
- `_` unsupported arrangement — driven (neg_unsupported_src, passing; sweep)
- is_high Q override — driven (metamorphic, passing)
- Ok Word layout — driven (invariant, passing)
- WIDE Vn-as-narrow — driven (diff_llvm_mc_wide, failing bug)
- extra / dest Ta / GPR Rm / Rm Tb — driven (failing bugs)

Sweep round 1/1 closed: unsupported-src property added and passing; Rm Tb mismatch added and filed as B5.

| Function | Indexed | Property | Executed |
|----------|---------|----------|----------|
| encode_neon_three_diff | yes | 10 | yes (cargo test --lib) |
