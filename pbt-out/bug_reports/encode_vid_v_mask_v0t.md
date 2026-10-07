# Bug: encode_vid_v ignores trailing v0.t and hardcodes vm=1
**Law:** ∀ vd ∈ {0..31}. let mc = llvm-mc("vid.v v{vd}, v0.t"); let sut = encode_vid_v([Reg("v{vd}"), Symbol("v0.t")]). (mc = Ok(w) ⇒ sut = Ok(w)) ∧ (mc = Err ⇒ sut = Err)
**Impact:** RISC-V V 1.0 vid.v is maskable. llvm-mc `-triple=riscv64 -mattr=+v` encodes `vid.v v1, v0.t` as 0x5008a0d7 (vm=0) and rejects `vid.v v0, v0.t` (destination overlaps the mask register). The SUT ignores the mask token and always emits unmasked vid.v (vm=1). A caller requesting a masked element-index write gets unmasked machine code; `vid.v v0, v0.t` is silently accepted.
**Function:** encode_vid_v
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:182
**Detected by:** Differential — llvm-mc RISC-V V 1.0 masked vid.v
**Minimal input:** encode_vid_v([Reg("v0"), Symbol("v0.t")])
**Expected:** Err (llvm-mc: The destination vector register group cannot overlap the mask register). For vd ∈ {1..31}, Ok(Word) with vm=0 matching llvm-mc (vid.v v1, v0.t = 0x5008a0d7).
**Actual:** Ok(Word(0x5208a057)) — same encoding as unmasked `vid.v v0`. Likewise encode_vid_v([Reg("v1"), Symbol("v0.t")]) = Ok(Word(0x5208a0d7)) vs llvm-mc 0x5008a0d7.
**Severity:** medium (documented by the author)
**Root cause:** vector.rs:183-186 reads only operand 0 and hardcodes `(1u32 << 25)` (vm=1). Trailing Symbol("v0.t") is never inspected. Dispatcher TODO encoder/mod.rs:962 admits masked variants are not yet supported.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:183`
```rust
    let vd = get_vreg(operands, 0)?;
    // vs2=0 (bits 24:20), funct6=010100, vm=1
    let word = (0b010100u32 << 26) | (1u32 << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Honour a trailing v0.t by clearing vm, reject vd=v0 when masked, and reject any other extra operand.
```rust
    let vd = get_vreg(operands, 0)?;
    let vm = match operands.get(1) {
        None => 1u32,
        Some(Operand::Symbol(s)) if s.eq_ignore_ascii_case("v0.t") => {
            if operands.len() != 2 {
                return Err(format!("vid.v masked form expects 2 operands, got {}", operands.len()));
            }
            if vd == 0 {
                return Err("vid.v vd cannot overlap mask register v0".into());
            }
            0u32
        }
        other => return Err(format!("unexpected operand 1 for vid.v: {:?}", other)),
    };
    let word = (0b010100u32 << 26) | (vm << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vid_v_regression_mask_v0_overlap -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vid_v_pbt::encode_vid_v_mask_v0t_diff_llvm_mc' (2890489) panicked at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:210:1:
Test failed: SUT encoded vid.v v0, v0.t as 0x5208a057 but llvm-mc rejected: llvm-mc error: <stdin>:1:7: error: The destination vector register group cannot overlap the mask register.
vid.v v0, v0.t
      ^
 at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:302.
minimal failing input: vd = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
