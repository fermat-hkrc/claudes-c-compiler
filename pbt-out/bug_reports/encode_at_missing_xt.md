# Bug: encode_at encodes AT without Xt as AT XZR
**Law:** If llvm-mc / GNU as reject `at s1e1r` (Xt required), then encode_at([], "s1e1r") must return Err
**Impact:** `at s1e1r` with a missing address register assembles as `at s1e1r, xzr` (0xd508781f). A dropped operand becomes a real address-translate instruction against XZR instead of an assembler error.
**Function:** encode_at
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:426
**Detected by:** Negative/Error Contract (missing Xt on AT; llvm-mc rejects)
**Minimal input:** encode_at(&[], "s1e1r")  (assembly: `at s1e1r`)
**Expected:** Err (llvm-mc: "specified at op requires a register"; gas: "comma expected between operands at operand 2")
**Actual:** Ok(Word(0xd508781f))  // AT S1E1R, XZR
**Severity:** medium
**Root cause:** system.rs:432-434 default Rt to 31 (XZR) whenever raw_operands contains no comma. Every ARM AT op requires Xt, so this default is never valid.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:432`
```rust
    } else {
        31
    };
```
**Suggested fix:** Require a register operand for every AT op.
```rust
    } else {
        return Err("at: operation requires a register".to_string());
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_missing_xt -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_neg_missing_reg' (2583788) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:337:1:
Test failed: AT without register must Err (llvm-mc rejects at s1e1r); SUT raw "s1e1r": Word(3574102047).
minimal failing input: raw = "s1e1r"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_at_pbt.rs
