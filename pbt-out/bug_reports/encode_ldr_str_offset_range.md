# Bug: encode_ldr_str truncates out-of-range offsets into imm9
**Law:** An offset must be a valid unsigned pimm (imm12 * scale in range) or a simm9 in [-256, 255]. llvm-mc/gas reject `strb w0, [x0, #-257]`.
**Impact:** `strb w0, [x0, #-257]` encodes as unscaled STURB with imm9 = (-257 as i32) & 0x1FF = 0x0FF (unsigned 255), word 0x380FF000 — a different in-range offset, not an error.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc range check
**Minimal input:** `encode_ldr_str([Reg("w0"), Mem{base:"x0", offset:-257}], is_load=false, size=0, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x380FF000))`
**Severity:** medium
**Root cause:** After the unsigned-pimm path fails, the unscaled fallback masks the offset to 9 bits with no range check (load_store.rs:81). Pre/post-index do the same.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:77`
```rust
            let imm9 = (*offset as i32) & 0x1FF;
```
**Suggested fix:** Reject offsets outside [-256, 255] for the unscaled/pre/post forms.
```rust
            if *offset < -256 || *offset > 255 {
                return Err("ldr/str: offset out of range".to_string());
            }
            let imm9 = (*offset as i32) & 0x1FF;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_imm9_range -- --test-threads=1
```
**Raw output:**
```text
Test failed: out-of-range Mem offset -257 must Err (llvm-mc range); got Ok(Word(940568576))
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0, extra = Reg("x0"), prepost = 0, which_off = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
