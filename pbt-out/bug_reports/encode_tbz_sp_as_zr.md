# Bug: encode_tbz accepts SP/WSP as Rt and encodes it as ZR
**Law:** TBZ/TBNZ Rt must be a GPR (W/X including ZR/LR), not SP/WSP. encode_tbz([Reg("sp"|"wsp"), Imm(bit), Symbol(s)], is_nz) must be Err.
**Impact:** `tbz sp, #0, L` is encoded as `tbz xzr, #0, L` (Rt=31). llvm-mc and gas reject SP. A stack-pointer test-and-branch is silently retargeted at ZR.
**Function:** encode_tbz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:254
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tbz([Reg("sp"), Imm(0), Symbol("L")], false)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x3600001f, reloc: TstBr14 symbol=L addend=0 }) — SP mapped to register 31
**Severity:** medium
**Root cause:** compare_branch.rs:255 calls get_reg, which uses parse_reg_num mapping both SP and XZR to 31; encode_tbz never distinguishes them and discards the width flag.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:255`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject SP/WSP as Rt.
```rust
    let (rt, _) = get_reg(operands, 0)?;
    if let Some(Operand::Reg(name)) = operands.get(0) {
        let l = name.to_ascii_lowercase();
        if l == "sp" || l == "wsp" {
            return Err(format!("tbz: SP is not a valid Rt: {}", name));
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_sp_as_zr -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tbz_pbt::encode_tbz_neg_wrong_reg stdout ----
Test failed: tbz sp , #0, L must Err (llvm-mc rejects SP/WSP) at src/backend/arm/assembler/encoder/encode_tbz_pbt.rs:541.
minimal failing input: which = 0, is_nz = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
