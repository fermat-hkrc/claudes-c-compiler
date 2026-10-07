# Bug: encode_vid_v ignores extra operands
**Law:** ∀ vd ∈ {0..31}, extra ∈ Operand \ {Symbol("v0.t")}. encode_vid_v([Reg("v{vd}"), extra]) is Err
**Impact:** A second (or later) operand other than v0.t is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `operand must be v0.t` / `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit unmasked vid.v encoding for the first operand instead of an assembler error.
**Function:** encode_vid_v
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:182
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vid_v([Reg("v0"), Imm(0)])
**Expected:** Err (complete vd form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x5208a057)) — same encoding as `vid.v v0`
**Severity:** medium
**Root cause:** vector.rs:183-186 encode_vid_v reads only operand 0 via get_vreg and never checks operands.len() == 1, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:183`
```rust
    let vd = get_vreg(operands, 0)?;
    // vs2=0 (bits 24:20), funct6=010100, vm=1
    let word = (0b010100u32 << 26) | (1u32 << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly one vector register (unmasked) or vd plus v0.t (masked).
```rust
    if operands.len() != 1 {
        return Err(format!("vid.v expects 1 operand, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vid_v_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vid_v_pbt::encode_vid_v_neg_extra' (2890497) panicked at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:210:1:
Test failed: extra operand Imm(0) must Err for vid.v (llvm-mc rejects extra except v0.t); got Ok(Word(1376297047)) at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:269.
minimal failing input: vd = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
