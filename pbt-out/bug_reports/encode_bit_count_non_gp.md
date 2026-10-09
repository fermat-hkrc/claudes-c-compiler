# Bug: encode_bit_count accepts xmm/mm/st/ymm via reg_num aliasing

**Law:** ∀ m ∈ {lzcntl,tzcntl,popcntl}, operands must be GP r32; non-GP names (xmm/mm/st/ymm) must Err even when `reg_num` would map them to a 3-bit code
**Impact:** `lzcntl %xmm0, %eax` encodes as `lzcntl %eax, %eax` ([f3,0f,bd,c0]). Invalid assembly is accepted and produces a GP instruction the programmer did not write.
**Function:** encode_bit_count
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972
**Detected by:** Negative/error contract vs llvm-mc (encode_bit_count_neg_non_gp)
**Minimal input:** `lzcntl %xmm0, %eax`
**Expected:** Err (non-GP register)
**Actual:** Ok([0xf3, 0x0f, 0xbd, 0xc0])
**Severity:** high
**Root cause:** Same as wrong-width: `reg_num` in `registers.rs:4-14` aliases `xmm0`/`mm0`/`st(0)`/`ymm0` to code 0. `encode_bit_count` never checks `is_xmm` / `is_mm` / GP-only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Suggested fix:** Reject non-GP names before encoding:
```rust
fn is_gp32(name: &str) -> bool {
    matches!(name, "eax"|"ecx"|"edx"|"ebx"|"esp"|"ebp"|"esi"|"edi")
}
// in arm:
if !is_gp32(&src.name) || !is_gp32(&dst.name) {
    return Err(format!("{} requires GP r32 registers", mnemonic));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bit_count_regression_rejects_xmm0_eax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP `lzcntl %xmm0, %eax` → [f3, 0f, bd, c0]; bit-count requires GP r32 (Intel SDM; reg_num must not alias xmm/mm/st/ymm).
minimal failing input: m = "lzcntl", bad = "xmm0", d = "eax", flip = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs (test_encode_bit_count_regression_rejects_xmm0_eax)
