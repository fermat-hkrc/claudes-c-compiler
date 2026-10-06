# Bug: encode_fma ignores a 6th operand
**Law:** A sixth operand after rd, rs1, rs2, rs3, rm must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fma.
**Impact:** Malformed `fmadd.s f0, f0, f0, f0, rne, 0` still assembles as `fmadd.s f0, f0, f0, f0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid R4-type FMA word instead of diagnosing the extra operand.
**Function:** encode_fma
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:175
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fma([Reg("f0"), Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b1000011, 0)
**Expected:** Err
**Actual:** Ok(Word(67)) which is 0x00000043, the encoding of fmadd.s f0, f0, f0, f0, rne
**Severity:** medium
**Root cause:** float.rs:176-191 reads only operands 0..4 via get_freg / optional RoundingMode and returns Ok without checking operands.len() > 5, so a 6th operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:191`
```rust
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject more than five operands before packing the R4-type word.
```rust
    if operands.len() > 5 {
        return Err("fma: unexpected extra operand".to_string());
    }
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    let rs2 = get_freg(operands, 2)?;
    let rs3 = get_freg(operands, 3)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fma_pbt -- --test-threads=1
cargo test --lib test_encode_fma_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 6th operand must Err for fmadd.s f0, f0, f0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(67)) at src/backend/riscv/assembler/encoder/encode_fma_pbt.rs:569.
minimal failing input: (mn, opc, fmt) = (
    "fmadd.s",
    67,
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", rs3 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
