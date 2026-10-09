# Bug: encode_mov_mem_reg rejects es/cs/ss/ds segment overrides
**Law:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, valid mem→GP mov. SUT bytes equal llvm-mc including the segment-override prefix (26/2E/36/3E/64/65)
**Impact:** Assembler rejects or mishandles valid AT&T forms like `movl %es:(%eax), %ebx` that appear in OS/kernel code; i686 already has `emit_segment_prefix` for all six overrides, but this path does not use it
**Function:** encode_mov_mem_reg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:194
**Detected by:** Differential — llvm-mc i686 (segment override forms)
**Minimal input:** `movb %es:(%eax), %al` (also `movl %es:(%eax), %ebx`)
**Expected:** bytes `[0x26, 0x8a, 0x00]` (movb) / `[0x26, 0x8b, 0x18]` (movl %es:(%eax), %ebx)
**Actual:** `Err("unsupported segment: es")`
**Severity:** high
**Root cause:** gp_integer.rs:198-203 inlines a fs/gs-only match and returns Err for every other segment, instead of calling `emit_segment_prefix` (core.rs:31-42) which already emits 26/2E/36/3E/64/65
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:198`
```rust
        if let Some(ref seg) = mem.segment {
            match seg.as_str() {
                "fs" => self.bytes.push(0x64),
                "gs" => self.bytes.push(0x65),
                _ => return Err(format!("unsupported segment: {}", seg)),
            }
        }
```
**Suggested fix:** Replace the inline match with the shared helper (same pattern as the x86-64 sibling path conceptually; i686 helper returns `()`):
```rust
        self.emit_segment_prefix(mem);
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_mem_reg_diff_llvm_mc_segment -- --test-threads=1
cargo test --lib encode_mov_mem_reg_regression_es_segment_prefix -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid segment form `movb %es:(%eax), %al`: unsupported segment: es; i686 MOV mem→reg must emit segment override (core.rs emit_segment_prefix; Intel SDM 2.1.1). Body only accepts fs/gs..
minimal failing input: seg = "es", base = "eax", disp = 0, width = 1, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs (encode_mov_mem_reg_regression_es_segment_prefix)
