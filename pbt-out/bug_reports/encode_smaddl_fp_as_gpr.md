# Bug: encode_smaddl accepts FP/SIMD registers as GPRs
**Law:** SMADDL operands are GPRs (`Xd, Wn, Wm, Xa`). An FP/SIMD register (d/s/q/v/h/b) in any slot must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts `smaddl d0, w1, w2, x3` and emits the same word as `smaddl x0, w1, w2, x3`. A mistyped SIMD register is silently treated as the same-numbered GPR.
**Function:** encode_smaddl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:653
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("w1"), Reg("w2"), Reg("x3")]`
**Expected:** `Err`
**Actual:** `Ok(Word(0x9b220c20))` — d0 parses as register 0
**Severity:** medium
**Root cause:** encoder/mod.rs:276 parse_reg_num accepts prefixes d/s/q/v/h/b; encode_smaddl:654-657 does not reject FP/SIMD names.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject FP/SIMD register names in every SMADDL slot.
```rust
    if is_fp_reg(name) {
        return Err(format!("smaddl: FP/SIMD register {} is not a GPR", name));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_fp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_smaddl_pbt::encode_smaddl_neg_fp stdout ----
Test failed: FP/SIMD register d0 is not a valid SMADDL operand (which=0) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:521.
minimal failing input: which = 0, prefix = "d", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
