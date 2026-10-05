# Bug: encode_dsb encodes non-barrier operands as SY
**Law:** For every non-named / out-of-range first operand (Imm outside 0..=15, Reg, Mem, Cond, Shift, Label, Extend), if llvm-mc rejects the corresponding `dsb` assembly then encode_dsb([op]) must return Err
**Impact:** Invalid operands such as `dsb #-1` and `dsb x0` encode as `dsb sy` instead of an assembler error, silently inserting a full-system barrier.
**Function:** encode_dsb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:30
**Detected by:** Negative/Error Contract (wrong kind / oob imm; llvm-mc and GNU as reject)
**Minimal input:** encode_dsb(&[Operand::Imm(-1)])  (assembly: `dsb #-1`)
**Expected:** Err (llvm-mc: "barrier operand out of range")
**Actual:** Ok(Word(0xd5033f9f))  // encodes as dsb sy
**Severity:** medium
**Root cause:** system.rs:47 `_ => 0b1111` — Imm, Reg, Mem, and every other non-Barrier/non-Symbol kind take the SY default.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Reject non-barrier kinds and immediates outside 0..=15.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dsb immediate out of range: {}", n)),
        Some(_) => return Err("dsb: invalid operand".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dsb_pbt::encode_dsb_neg_wrong_kind_imm_oob' panicked at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:264:1:
Test failed: non-named / out-of-range operand must Err, got Ok(Word(3573759903)) at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:396.
minimal failing input: op = Imm(-1)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
