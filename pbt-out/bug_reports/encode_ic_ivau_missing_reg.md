# Bug: encode_ic encodes IC IVAU without Xt as IVAU XZR
**Law:** If llvm-mc / GNU as reject `ic ivau` (Xt required), then encode_ic("ivau") must return Err
**Impact:** `ic ivau` with a missing address register assembles as `ic ivau, xzr` (0xd50b753f). A dropped operand becomes a real cache-maintenance instruction against XZR instead of an assembler error.
**Function:** encode_ic
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:401
**Detected by:** Negative/Error Contract (missing Xt on IVAU; llvm-mc rejects)
**Minimal input:** encode_ic("ivau")  (assembly: `ic ivau`)
**Expected:** Err (llvm-mc: "specified ic op requires a register"; gas: "missing register at operand 2")
**Actual:** Ok(Word(0xd50b753f))  // IC IVAU, XZR
**Severity:** medium
**Root cause:** system.rs:407-408 default Rt to 31 (XZR) whenever raw_operands contains no comma, including for IVAU which requires Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:407`
```rust
    } else {
        31 // xzr
    };
```
**Suggested fix:** Require a register operand for IVAU.
```rust
    } else if op_name == "ivau" {
        return Err("ic: ivau requires a register".to_string());
    } else {
        31 // xzr
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ivau_missing -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ic_pbt::encode_ic_neg_ivau_missing_reg' (2565615) panicked at src/backend/arm/assembler/encoder/encode_ic_pbt.rs:345:1:
Test failed: IVAU without register must Err (llvm-mc rejects ic ivau); SUT raw "ivau": Word(3574297919).
minimal failing input: raw = "ivau"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ic_pbt.rs
