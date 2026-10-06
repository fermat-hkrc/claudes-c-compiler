# Bug: encode_tbz accepts FP/SIMD registers as Rt
**Law:** TBZ/TBNZ Rt must be a GPR. encode_tbz([Reg(dN|sN|qN|vN|hN|bN), Imm(bit), Symbol(s)], is_nz) must be Err.
**Impact:** `tbz d0, #0, L` is encoded as `tbz w0, #0, L` (Rt=0). llvm-mc and gas reject FP/SIMD Rt. A floating-point test-and-branch is silently retargeted at the same-numbered GPR.
**Function:** encode_tbz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:254
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tbz([Reg("d0"), Imm(0), Symbol("L")], false)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x36000000, reloc: TstBr14 symbol=L addend=0 }) — d0 parsed as register 0
**Severity:** medium
**Root cause:** compare_branch.rs:255 calls get_reg, which uses parse_reg_num accepting prefixes d/s/q/v/h/b; encode_tbz never checks is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:255`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject FP/SIMD Rt.
```rust
    let (rt, _) = get_reg(operands, 0)?;
    if let Some(Operand::Reg(name)) = operands.get(0) {
        if is_fp_reg(name) {
            return Err(format!("tbz: FP/SIMD register not valid Rt: {}", name));
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_fp_reg -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tbz_pbt::encode_tbz_neg_fp_reg stdout ----
Test failed: tbz d0 , #0, L must Err (llvm-mc rejects FP/SIMD Rt) at src/backend/arm/assembler/encoder/encode_tbz_pbt.rs:562.
minimal failing input: which = 2, n = 0, is_nz = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
