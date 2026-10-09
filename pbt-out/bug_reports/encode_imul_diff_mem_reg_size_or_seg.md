# Bug: encode_imul Mem→Reg misses 0x66 (imulw) and/or segment prefix
**Law:** `encode(imul mem, dst)` must match llvm-mc for width∈{2,4} including segment overrides.
**Impact:** Wrong machine code for memory-source IMUL.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:723
**Detected by:** Differential (encode_imul_diff_mem_reg)
**Minimal input:** width=2, mem=(%eax), dst=ax
**Expected:** 66 0f af … (and 26… with %es:)
**Actual:** missing 0x66 / missing segment
**Severity:** high
**Root cause:** gp_integer.rs:723-726 ignores size and emit_segment_prefix (same class as b1/b2).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:725`
```rust
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.encode_modrm_mem(dst_num, mem)
```
**Suggested fix:**
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.encode_modrm_mem(dst_num, mem)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_diff_mem_reg -- --test-threads=1
```
**Raw output:**
```text
minimal failing input: width = 2, form = 0, bi = 0, ii = 0, di = 0, scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
