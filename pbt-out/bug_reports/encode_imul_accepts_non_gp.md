# Bug: encode_imul accepts non-GP / mismatched-width registers via reg_num aliasing
**Law:** Two-operand IMUL register forms require general-purpose registers of the mnemonic width; non-GP names (xmm/mm/st/ymm) and cross-width pairs must be rejected, as llvm-mc rejects them.
**Impact:** Invalid assembly like `imull %xmm0, %eax` silently encodes as `imull %eax, %eax` (`0f af c0`), producing wrong machine code without an assembler error.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:717
**Detected by:** Negative/error (encode_imul_neg_unsupported_shape kind=2, encode_imul_neg_mismatched_or_non_gp)
**Minimal input:** ops = [Register("xmm0"), Register("eax")], mnemonic `imull` → Ok(`[0f, af, c0]`); llvm-mc rejects `imull %xmm0, %eax`
**Expected:** Err("bad register") or equivalent rejection
**Actual:** Ok with GP-aliased encoding (xmm0 → reg number 0)
**Severity:** medium
**Root cause:** `reg_num` returns Some for several non-GP names (or maps low bits); encode_imul never checks `reg_size` against `size` or rejects non-GP classes before encoding.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:717`
```rust
let src_num = reg_num(&src.name).ok_or("bad register")?;
let dst_num = reg_num(&dst.name).ok_or("bad register")?;
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
```
**Suggested fix:** Gate register forms on GP + matching width:
```rust
if reg_size(&src.name) != size || reg_size(&dst.name) != size {
    return Err("imul register width mismatch".into());
}
// and ensure reg_num is None for xmm/mm/st/ymm (or check is_gp_reg)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_neg_unsupported_shape -- --test-threads=1
cargo test --lib encode_imul_neg_mismatched_or_non_gp -- --test-threads=1
```
**Raw output:**
```text
Test failed: unsupported shape kind=2 must Err, got Ok(Some([0f, af, c0]))
minimal failing input: kind = 2, bi = 0, ni = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs (encode_imul_neg_unsupported_shape / encode_imul_neg_mismatched_or_non_gp)
