# Bug: encode_tbz rejects immediate PC-offset form `tbz/tbnz Rt, #bit, #imm`
**Law:** For every GPR Rt, every valid bit number, and every 4-byte-aligned offset in the ARM ARM TBZ/TBNZ range [-2^15, 2^15-4], encode_tbz([Reg(rt), Imm(bit), Imm(imm)], is_nz) must equal Word(llvm-mc("tbz/tbnz rt, #bit, #imm")).
**Impact:** GNU-style assembly that uses an explicit PC offset (`tbz x0, #0, #0`, `tbnz x0, #32, #4`, `tbz x0, #0, #-32768`) fails to assemble. The built-in assembler claims gas compatibility, so this is a missing encoding path; codegen currently emits `tbz xN, #bit, .Llabel` so the hole is latent for compiler output but user/hand-written `.s` files are rejected.
**Function:** encode_tbz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:254
**Detected by:** Differential — llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** encode_tbz([Reg("x0"), Imm(0), Imm(-32768)], false)  (`tbz x0, #0, #-32768`); also Imm(0) and Imm(4)
**Expected:** Word matching llvm-mc: `tbz x0, #0, #0` → 0x36000000, `tbz w0, #0, #0` → 0x36000000, `tbnz x0, #32, #4` → 0xb7000020, `tbz x0, #0, #-32768` → 0x36040000
**Actual:** Err("expected symbol at operand 2, got Some(Imm(-32768))") because encode_tbz only calls get_symbol for operand 2 and never encodes the ARM ARM imm14 field.
**Severity:** medium
**Root cause:** compare_branch.rs:257 calls get_symbol for the branch target and never matches Operand::Imm, so the imm14 field (bits[18:5]) is always left 0 and immediate offsets are rejected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:257`
```rust
    let (sym, addend) = get_symbol(operands, 2)?;
```
**Suggested fix:** When operand 2 is Imm, range-check a 4-byte-aligned offset in [-32768, 32764], pack imm14, and return Word.
```rust
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        if *imm % 4 != 0 || *imm < -32768 || *imm > 32764 {
            return Err(format!("tbz offset out of range: {}", imm));
        }
        let imm14 = ((*imm as i32) >> 2) as u32 & 0x3fff;
        let word = (b5 << 31) | (0b011011 << 25) | (op << 24) | (b40 << 19) | (imm14 << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }
    let (sym, addend) = get_symbol(operands, 2)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_imm_offset -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tbz_pbt::encode_tbz_diff_imm_llvm_mc stdout ----
Test failed: SUT rejected valid tbz x0, #0, #-32768: Err("expected symbol at operand 2, got Some(Imm(-32768))").
minimal failing input: (rt, bit) = ("x0", 0), is_nz = false, imm = -32768
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
