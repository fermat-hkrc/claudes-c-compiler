# Bug: encode_push16 rejects segmented memory pushw
**Law:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, d ∈ i64. encode_push16([Mem(seg:base+d)]) must equal llvm-mc `pushw %seg:d(%base)` = segment override + 0x66 + FF /6 + ModR/M.
**Impact:** Segment-overridden 16-bit memory pushes cannot be assembled (e.g. `pushw %es:(%eax)`).
**Function:** encode_push16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:382
**Detected by:** Differential — llvm-mc i686 (encode_push16_diff_mem_segment)
**Minimal input:** `pushw %es:(%eax)` → Memory segment=es base=eax
**Expected:** `[0x26, 0x66, 0xff, 0x30]`
**Actual:** `Err("unsupported pushw operand")`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400` — Memory (including segmented) falls through to catch-all Err. Same missing Memory arm as bare mem; fix must call `emit_segment_prefix` before 0x66 + FF /6.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Memory arm with segment prefix, 0x66, FF /6:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push16_diff_mem_segment -- --test-threads=1
cargo test --lib test_encode_push16_regression_mem_segment_unsupported -- --test-threads=1
```
**Raw output:**
```text
regression: encode_push16 must accept segmented mem, got Err(unsupported pushw operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push16_pbt.rs::test_encode_push16_regression_mem_segment_unsupported
