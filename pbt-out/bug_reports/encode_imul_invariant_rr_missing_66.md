# Bug: encode_imul RR invariant fails — imulw missing 0x66
**Law:** RR encoding is [0x66 if width=2] 0F AF modrm(3,dst,src).
**Impact:** Same as missing operand-size prefix on imulw RR.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:719
**Detected by:** Algebraic invariant (encode_imul_invariant_rr_opcode)
**Minimal input:** width=2, src=ax, dst=ax
**Expected:** first byte 0x66
**Actual:** first byte 0x0F
**Severity:** high
**Root cause:** gp_integer.rs:719 no size==2 prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:719`
```rust
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.bytes.push(self.modrm(3, dst_num, src_num));
```
**Suggested fix:**
```rust
if size == 2 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.bytes.push(self.modrm(3, dst_num, src_num));
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_invariant_rr_opcode -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)` left: `Some(15)`, right: `Some(102)`: imulw must emit 0x66
minimal failing input: width = 2, si = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
