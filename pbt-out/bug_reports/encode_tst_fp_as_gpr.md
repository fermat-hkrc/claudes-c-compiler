# Bug: encode_tst accepts FP/SIMD registers as GPRs
**Law:** TST Rn, Rm must be integer GPRs; FP/SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `tst d0, d0` is encoded as `tst x0, x0` (0xea00001f) because parse_reg_num maps the numeric suffix and encode_tst treats a non-W first operand as 64-bit. Invalid FP assembly becomes a GPR TST. llvm-mc rejects the operand.
**Function:** encode_tst
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:37
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tst([Reg("d0"), Reg("d0")])
**Expected:** Err
**Actual:** Ok(Word(0xea00001f)) — d0 parses as register number 0, prepended XZR sets sf=1
**Severity:** medium
**Root cause:** encode_tst never calls is_fp_reg; parse_reg_num accepts prefixes d/s/q/v/h/b (mod.rs:272) and returns the element number, which then occupies Rn/Rm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:272`
```rust
                'x' | 'w' | 'd' | 's' | 'q' | 'v' | 'h' | 'b' => {
                    let num: u32 = name[1..].parse().ok()?;
                    if num <= 31 { Some(num) } else { None }
```
**Suggested fix:** Reject FP/SIMD names in encode_tst.
```rust
    if operands.iter().any(|o| matches!(o, Operand::Reg(r) if is_fp_reg(r))) {
        return Err("tst: FP/SIMD register is not a GPR".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_fp_reg -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tst_pbt::encode_tst_neg_fp_reg stdout ----
Test failed: tst FP/SIMD register must Err (llvm-mc: invalid operand) at src/backend/arm/assembler/encoder/encode_tst_pbt.rs:696.
minimal failing input: n = 0, m = 0, pref = 0, as_rm = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tst_pbt.rs
