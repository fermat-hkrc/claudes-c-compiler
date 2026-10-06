# Bug: encode_ldr_str encodes pre/post writeback when Rt == Rn
**Law:** Pre-index and post-index LDR/STR with Rt == Rn (and Rn != SP) are UNPREDICTABLE. llvm-mc/gas reject `ldr x0, [x0, #8]!`.
**Impact:** `ldr x0, [x0, #8]!` assembles as 0xF8408C00 instead of an error. The resulting instruction is architecturally UNPREDICTABLE.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects writeback Rt==Rn
**Minimal input:** `encode_ldr_str([Reg("x0"), MemPreIndex{base:"x0", offset:8}], is_load=true, size=0b11, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0xF8408C00))`
**Severity:** medium
**Root cause:** The pre-index (load_store.rs:96-104) and post-index (107-115) paths encode imm9 writeback with no Rt==Rn check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:91`
```rust
        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load { 0b01 } else { 0b00 };
            let word = ((actual_size << 30) | (0b111 << 27) | (v << 26)) | (opc << 22)
                | ((imm9 as u32 & 0x1FF) << 12) | (0b11 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }
```
**Suggested fix:** Reject writeback when Rt equals Rn and Rn is not SP.
```rust
            if rt == rn && rn != 31 {
                return Err("ldr/str: writeback with Rt==Rn is unpredictable".to_string());
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_writeback_overlap -- --test-threads=1
```
**Raw output:**
```text
LDR X0, [X0, #8]! must Err; writeback Rt==Rn is unpredictable (llvm-mc rejects it)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
