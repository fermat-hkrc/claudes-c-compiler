# Bug: encode_dsb encodes omitted option as SY
**Law:** encode_dsb([]) must return Err, matching GNU as and llvm-mc which reject omitted-operand `dsb`
**Impact:** A `dsb` with no operand, rejected by gas and llvm-mc, is encoded as a full-system barrier (`dsb sy`). Callers that drop the option accidentally get stronger ordering than they wrote, with no assembler error.
**Function:** encode_dsb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:30
**Detected by:** Negative/Error Contract (empty operand list; llvm-mc and GNU as reject)
**Minimal input:** encode_dsb(&[])  (assembly: `dsb`)
**Expected:** Err (GNU as: "missing immediate expression at operand 1"; llvm-mc: "too few operands for instruction dsb")
**Actual:** Ok(Word(0xd5033f9f))  // encodes as dsb sy
**Severity:** medium
**Root cause:** system.rs:47 `_ => 0b1111` — `operands.first()` is None and defaults CRm to SY.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Return Err when the operand list is empty.
```rust
        None => return Err("dsb requires a barrier option".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_empty -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dsb_pbt::encode_dsb_neg_empty' panicked at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:264:1:
Test failed: empty operands must Err (gas/llvm-mc reject omitted dsb option) at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:381.
minimal failing input: _n = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
