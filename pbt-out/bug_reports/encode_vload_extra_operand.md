# Bug: encode_vload ignores extra operands
**Law:** ∀ vd ∈ {v0..v31}, rs1 ∈ GPR names, extra ∈ Operand. encode_vload([Reg(vd), Mem{base:rs1, offset:0}, extra], width, lumop) = Err(_)
**Impact:** A third (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `expected '.t' suffix` or `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit unit-stride load for the first two operands instead of an assembler error. A trailing `v0.t` mask token is also dropped, so a masked load is silently encoded as unmasked (vm=1).
**Function:** encode_vload
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:81
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vload([Reg("v0"), Mem{base:"x0", offset:0}, Imm(0)], width=0b000, lumop=0)
**Expected:** Err (complete vd, (rs1) form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x02000007)) — same encoding as `vle8.v v0, (x0)`
**Severity:** medium
**Root cause:** vector.rs:82-99 encode_vload reads only operands 0 and 1 via get_vreg and the rs1 match and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:82`
```rust
    let vd = get_vreg(operands, 0)?;
    // The second operand should be a memory operand (rs1) like (a1)
    let rs1 = match operands.get(1) {
        Some(Operand::Mem { base, offset: 0 }) => {
            reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?
        }
        Some(Operand::Reg(name)) => {
            // Parenthesized register may be parsed differently
            reg_num(name).ok_or_else(|| format!("invalid register: {}", name))?
        }
        other => return Err(format!("expected (rs1) at operand 1, got {:?}", other)),
    };
    // vm=1 means unmasked (no v0.t)
    let vm: u32 = 1;
    // nf=000 (single segment), mew=0, mop=00 (bits 31:26 = 0)
    let word = (vm << 25)
        | (lumop << 20) | (rs1 << 15) | (width << 12) | (vd << 7) | OP_LOAD_FP;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vle*.v/vlm.v expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vload_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vload_pbt::encode_vload_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_vload_pbt.rs:325:1:
Test failed: extra operand Imm(0) must Err for vload (llvm-mc rejects extra); got Ok(Word(33554439)) at src/backend/riscv/assembler/encoder/encode_vload_pbt.rs:476.
minimal failing input: vd = 0, rs1 = "x0", extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
