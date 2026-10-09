# Bug: encode_alu accepts non-GP register names (xmm/mm/st) via reg_num aliasing
**Law:** ALU r/m forms are GP-only (Intel SDM); names outside the GP set that llvm-mc rejects must return Err.
**Impact:** `addb %al, %xmm0` encodes as `addb %al, %al` (`[00, c0]`) — silent mis-assembly if a bad operand slips through.
**Function:** encode_alu
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:496
**Detected by:** Negative/error contract (llvm-mc reject set)
**Minimal input:** `addb %al, %xmm0`
**Expected:** Err(...)
**Actual:** Ok(`[0x00, 0xc0]`)
**Severity:** high
**Root cause:** `reg_num` maps xmm0/mm0/st(0) onto 0..7; encode_alu never distinguishes GP from SIMD/x87 names.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:496`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
```
**Suggested fix:** Reject non-GP names (check against the GP set for the mnemonic width, or a dedicated `is_gp_reg` helper) before calling `reg_num`.
```rust
                if !is_gp_reg(&src.name) || !is_gp_reg(&dst.name) {
                    return Err(format!("non-GP register in {mnemonic}"));
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib 'backend::i686::assembler::encoder::encode_alu_pbt::encode_alu_neg_non_gp' -- --test-threads=1 --nocapture
```
**Raw output:**
```text
Test failed: SUT accepted non-GP ALU RR `addb %al, %xmm0` → [00, c0]; ALU r/m forms are GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7.
minimal failing input: op_i = 0, ni = 0, gi = 0, width = 1, non_gp_as_src = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_alu_pbt.rs (encode_alu_neg_non_gp / encode_alu_regression can be added; property is the witness)
