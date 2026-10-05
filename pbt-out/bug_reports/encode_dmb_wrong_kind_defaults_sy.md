# Bug: encode_dmb encodes non-barrier operands as SY
**Law:** For every first operand that is not a named Barrier/Symbol and not Imm in 0..=15, encode_dmb([op]) must return Err (GNU as / llvm-mc reject registers, memory, and `#imm` outside 0..=15)
**Impact:** `dmb x0`, `dmb #-1`, `dmb #16`, and similar invalid operands encode as `dmb sy`. Out-of-range immediates and wrong operand kinds become a full-system barrier instead of an assembler error.
**Function:** encode_dmb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:6
**Detected by:** Negative/Error Contract (wrong kind / imm out of 0..=15; llvm-mc and GNU as reject)
**Minimal input:** encode_dmb(&[Operand::Imm(-1)])  (assembly: `dmb #-1`)
**Expected:** Err (GNU as: "immediate value out of range 0 to 15"; llvm-mc: "barrier operand out of range")
**Actual:** Ok(Word(0xd5033fbf))  // dmb sy
**Severity:** medium
**Root cause:** system.rs:23 `_ => 0b1111` — Imm, Reg, Mem, Cond, and every other non-Barrier/non-Symbol kind take the default SY encoding instead of returning Err. Out-of-range immediates are therefore indistinguishable from `dmb sy`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:23`
```rust
        _ => 0b1111,
```
**Suggested fix:** Reject non-barrier kinds and immediates outside 0..=15.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dmb immediate out of range: {}", n)),
        Some(_) => return Err("dmb: invalid operand".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dmb_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dmb_pbt::encode_dmb_neg_wrong_kind_imm_oob' panicked at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:264:1:
Test failed: non-named / out-of-range operand must Err, got Ok(Word(3573759935)) at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:396.
minimal failing input: op = Imm(-1)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
