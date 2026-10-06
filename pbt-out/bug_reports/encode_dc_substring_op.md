# Bug: encode_dc matches DC op names by substring
**Law:** If llvm-mc / GNU as reject `dc <name>, Xt` for a name that is not exactly one of {civac, cvac, cvap, cvau, ivac, zva}, then encode_dc must return Err
**Impact:** Typos and other ARM DC names that merely contain an implemented token are assembled as the wrong cache op. `dc civacs, x0` becomes DC CIVAC; `dc gzva, x0` (MTE GZVA) becomes DC ZVA (zero cache line) instead of being rejected.
**Function:** encode_dc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:564
**Detected by:** Negative/Error Contract (unknown op; llvm-mc rejects)
**Minimal input:** encode_dc([Symbol("civacs"), Reg("x0")], "civacs, x0")  (assembly: `dc civacs, x0`)
**Expected:** Err (llvm-mc: unknown DC operation; gas: unknown operation name)
**Actual:** Ok(Word(0xd50b7e20))  // DC CIVAC, x0
**Severity:** medium
**Root cause:** system.rs:583 uses `op.contains("civac")` (and later `contains("cvac")` / `contains("zva")` etc.) instead of an exact match, so any string that has those letters as a substring encodes as that op.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:583`
```rust
    if op.contains("civac") {
```
**Suggested fix:** Match the trimmed op name exactly, not as a substring.
```rust
    if op.trim() == "civac" {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_gzva -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dc_pbt::encode_dc_neg_unknown_op' (2568667) panicked at src/backend/arm/assembler/encoder/encode_dc_pbt.rs:455:1:
Test failed: unknown DC op "civacs" must Err (llvm-mc rejects dc civacs, x0): Word(3574300192).
minimal failing input: s = "civacs"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dc_pbt.rs
