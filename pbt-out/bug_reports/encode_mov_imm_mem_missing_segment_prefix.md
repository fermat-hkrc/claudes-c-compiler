# Bug: encode_mov_imm_mem omits all segment-override prefixes
**Law:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, valid imm→mem mov. SUT bytes equal llvm-mc including the segment-override prefix (26/2E/36/3E/64/65)
**Impact:** Assembler silently drops segment overrides on forms like `movb $0, %es:(%eax)` and `movl $1, %fs:(%eax)`, producing wrong machine code that reads/writes the wrong segment — a silent correctness bug in OS/kernel-style AT&T asm
**Function:** encode_mov_imm_mem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:238
**Detected by:** Differential — llvm-mc i686 (segment override forms)
**Minimal input:** `movb $0, %es:(%eax)` (also `movl $1, %fs:(%eax)`, `movl $1, %es:(%eax)`)
**Expected:** bytes `[0x26, 0xc6, 0x00, 0x00]` for movb $0, %es:(%eax); `[0x64, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00]` for movl $1, %fs:(%eax)
**Actual:** `[0xc6, 0x00, 0x00]` / `[0xc7, 0x00, 0x01, 0x00, 0x00, 0x00]` — no override prefix at all (including fs/gs)
**Severity:** high
**Root cause:** gp_integer.rs:239-247 pushes optional 0x66 then C6/C7 and calls encode_modrm_mem without ever calling `emit_segment_prefix` (core.rs:31-42). Unlike encode_mov_reg_mem/encode_mov_mem_reg (which at least inline fs/gs), this path emits zero segment prefixes. The x86-64 sibling calls `emit_segment_prefix(mem)?` first.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:239`
```rust
    fn encode_mov_imm_mem(&mut self, imm: &ImmediateValue, mem: &MemoryOperand, size: u8) -> Result<(), String> {
        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0xC6);
        } else {
            self.bytes.push(0xC7);
        }
        self.encode_modrm_mem(0, mem)?;
```
**Suggested fix:** Emit the shared segment prefix before the operand-size override / opcode:
```rust
    fn encode_mov_imm_mem(&mut self, imm: &ImmediateValue, mem: &MemoryOperand, size: u8) -> Result<(), String> {
        self.emit_segment_prefix(mem);
        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0xC6);
        } else {
            self.bytes.push(0xC7);
        }
        self.encode_modrm_mem(0, mem)?;
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_imm_mem_diff_llvm_mc_segment -- --test-threads=1
cargo test --lib test_encode_mov_imm_mem_regression_missing_es_prefix -- --test-threads=1
cargo test --lib test_encode_mov_imm_mem_regression_missing_fs_prefix -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[198, 0, 0]`,
 right: `[38, 198, 0, 0]`: segment diff `movb $0, %es:(%eax)`: sut=[c6, 00, 00] mc=[26, c6, 00, 00]
minimal failing input: seg = "es", base = "eax", disp = 0, width = 1, imm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs (`test_encode_mov_imm_mem_regression_missing_es_prefix`, `test_encode_mov_imm_mem_regression_missing_fs_prefix`)
