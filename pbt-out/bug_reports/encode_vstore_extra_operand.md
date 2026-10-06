# Bug: encode_vstore ignores extra operands
**Law:** ∀ vs3 ∈ {v0..v31}, rs1 ∈ GPR names, extra ∈ Operand. encode_vstore([Reg(vs3), Mem{base:rs1, offset:0}, extra], width, sumop) = Err(_)
**Impact:** A third (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `operand must be v0.t`. Callers that pass a stray token get a valid-looking 32-bit unit-stride store for the first two operands instead of an assembler error. A trailing `v0.t` mask token is also dropped, so a masked store is silently encoded as unmasked (vm=1).
**Function:** encode_vstore
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:104
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vstore([Reg("v0"), Mem{base:"x0", offset:0}, Imm(0)], width=0b000, sumop=0)
**Expected:** Err (complete vs3, (rs1) form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x02000027)) — same encoding as `vse8.v v0, (x0)`
**Severity:** medium
**Root cause:** vector.rs:105-119 encode_vstore reads only operands 0 and 1 via get_vreg and the rs1 match and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:105`
```rust
    let vs3 = get_vreg(operands, 0)?;
    let rs1 = match operands.get(1) {
        Some(Operand::Mem { base, offset: 0 }) => {
            reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?
        }
        Some(Operand::Reg(name)) => {
            reg_num(name).ok_or_else(|| format!("invalid register: {}", name))?
        }
        other => return Err(format!("expected (rs1) at operand 1, got {:?}", other)),
    };
    let vm: u32 = 1;
    // nf=000, mew=0, mop=00 (bits 31:26 = 0)
    let word = (vm << 25)
        | (sumop << 20) | (rs1 << 15) | (width << 12) | (vs3 << 7) | OP_STORE_FP;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vse*.v/vsm.v expects 2 operands, got {}", operands.len()));
    }
    let vs3 = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vstore_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vstore_pbt::encode_vstore_neg_extra' (2875549) panicked at src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs:324:1:
Test failed: extra operand Imm(0) must Err for vstore (llvm-mc rejects extra); got Ok(Word(33554471)) at src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs:475.
minimal failing input: vs3 = 0, rs1 = "x0", extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
