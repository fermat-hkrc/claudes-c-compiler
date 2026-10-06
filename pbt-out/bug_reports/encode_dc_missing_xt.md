# Bug: encode_dc encodes a missing Xt as x0
**Law:** If llvm-mc / GNU as reject `dc <op>` with no register, then encode_dc([Symbol(op)], op) must return Err
**Impact:** `dc civac` (and cvac/cvap/cvau/ivac/zva) assembles as DC CIVAC, x0 instead of being rejected. A dropped operand is not diagnosed and silently targets x0.
**Function:** encode_dc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:564
**Detected by:** Negative/Error Contract (missing Xt; llvm-mc/gas reject)
**Minimal input:** encode_dc([Symbol("civac")], "civac")  (assembly: `dc civac`)
**Expected:** Err (llvm-mc rejects; gas: "comma expected between operands at operand 2")
**Actual:** Ok(Word(0xd50b7e20))  // DC CIVAC, x0
**Severity:** medium
**Root cause:** system.rs:578 defaults Rt to 0 when no register operand is present, so a missing Xt is encoded as x0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:578`
```rust
                0
```
**Suggested fix:** Require a register operand; do not default Rt to x0.
```rust
                return Err("dc: missing Xt register".into())
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_missing_xt -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dc_pbt::encode_dc_neg_missing_xt' (2568663) panicked at src/backend/arm/assembler/encoder/encode_dc_pbt.rs:455:1:
Test failed: DC civac without Xt must Err (llvm-mc rejects dc civac); SUT raw "civac": Word(3574300192).
minimal failing input: op = "civac"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dc_pbt.rs
