# Bug: encode_neon_rbit accepts a source without arrangement
**Law:** Vector RBIT syntax is `RBIT Vd.T, Vn.T`. The source must be a NEON register with arrangement; a bare `Vn` must be rejected with Err.
**Impact:** Operands `[v0.8b, v0]` encode as `rbit v0.8b, v0.8b`. llvm-mc rejects `rbit v0.8b, v0`. The dispatcher routes any dest `RegArrangement` into encode_neon_rbit, so this is caller-reachable from parsed assembly.
**Function:** encode_neon_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, t="8b" — dest `RegArrangement{v0,"8b"}`, src `Reg("v0")`
**Expected:** Err
**Actual:** Ok(Word) — `get_neon_reg` accepts `Operand::Reg` with an empty arrangement, and source `T` is ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_rbit_pbt::test_encode_neon_rbit_regression_bare_src
**Serial reconfirmation:** reproduced with `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`
