# Bug: encode_neon_rbit encodes SP as V31
**Law:** SP/WSP are stack-pointer aliases, not NEON vector registers. `rbit sp.8b, v0.8b` must be rejected with Err.
**Impact:** Parser treats `sp` as a register and `.8b` as a valid arrangement, so `sp.8b` reaches encode_neon_rbit. `parse_reg_num("sp")` returns 31, and the instruction encodes as `rbit v31.8b, v0.8b`. llvm-mc/gas reject the text. The same path applies to `wsp`/`xzr`/`wzr` (also mapped to 31) and `lr` (mapped to 30).
**Function:** encode_neon_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, t="8b", which=0 — `rbit sp.8b, v0.8b`
**Expected:** Err
**Actual:** Ok(Word) encoding Rd=31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_rbit_pbt::test_encode_neon_rbit_regression_sp
**Serial reconfirmation:** reproduced with `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`
