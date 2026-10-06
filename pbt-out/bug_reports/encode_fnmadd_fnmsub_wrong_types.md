# Bug: encode_fnmadd_fnmsub accepts mixed S/D, GPR, Q/V/B, and SP operands
**Law:** Scalar FNMADD/FNMSUB requires a matching Sd/Dd/Hd quadruple; mixed S/D, GPR, Q/V/B, and SP/WSP in any slot must be rejected.
**Impact:** The assembler encodes `fnmadd d0, s0, s0, s0`, `fnmadd x0, s1, s2, s3`, and `fnmadd s0, s1, s2, sp` as scalar fused multiply-add, producing a wrong-class machine-code word. llvm-mc and gas reject these operands.
**Function:** encode_fnmadd_fnmsub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:146
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fnmadd_fnmsub([Reg("d0"), Reg("s0"), Reg("s0"), Reg("s0")], false)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num accepts d/s/x/w/q/v/h/b/sp and the body never checks that all four registers share one FP class
**Severity:** medium
**Root cause:** fp_scalar.rs:147-154 get_reg/parse_reg_num accept any prefix; ftype is taken only from dest `starts_with('d')`, with no matching-class check on Rn/Rm/Ra.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:151`
```rust
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
```
**Suggested fix:** Require all four operands to be the same FP class (S, D, or H) and reject GPR/SP/Q/V/B.
```rust
    fn fp_class(name: &str) -> Option<char> {
        let c = name.chars().next()?.to_ascii_lowercase();
        matches!(c, 's' | 'd' | 'h').then_some(c)
    }
    let classes = [
        fp_class(&rd_name).ok_or("fnmadd/fnmsub needs FP dest")?,
        fp_class(match &operands[1] { Operand::Reg(r) => r, _ => "" }).ok_or("fnmadd/fnmsub needs FP Rn")?,
        fp_class(match &operands[2] { Operand::Reg(r) => r, _ => "" }).ok_or("fnmadd/fnmsub needs FP Rm")?,
        fp_class(match &operands[3] { Operand::Reg(r) => r, _ => "" }).ok_or("fnmadd/fnmsub needs FP Ra")?,
    ];
    if classes.iter().any(|c| *c != classes[0]) {
        return Err("fnmadd/fnmsub requires matching S/D/H operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_mixed_sd -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::fp_scalar::encode_fnmadd_fnmsub_pbt::encode_fnmadd_fnmsub_neg_wrong_types stdout ----
Test failed: FNMADD/FNMSUB requires matching Sd/Dd/Hd quadruples; dest=d0 n=s0 m=s0 a=s0 must Err
minimal failing input: (dest, src_n, src_m, src_a) = ("d0", "s0", "s0", "s0"), is_sub = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
