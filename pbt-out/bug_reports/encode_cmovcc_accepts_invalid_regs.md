# Bug: encode_cmovcc accepts r8, non-GP, and mismatched-width registers
**Law:** Intel CMOVcc is defined only for r16/r32 GP registers (and r/m16/r/m32 source). r8 (`%al`), non-GP (`%xmm0`/`%mm0`/`%st`/`%ymm0`), and mixed-width pairs (`%ax`/`%eax`) must be rejected — llvm-mc rejects them; `reg_num` aliasing must not silently map them onto GP encodings.
**Impact:** Invalid assembly is accepted and encodes as if the aliased GP register were used (`cmove %al,%al` → same bytes as `cmove %eax,%eax`). Callers that miss a type error get wrong-register conditional moves with no diagnostic.
**Function:** encode_cmovcc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1119
**Detected by:** Negative/error contract — encode_cmovcc_neg_r8 / encode_cmovcc_neg_non_gp / encode_cmovcc_neg_width_mismatch
**Minimal input:** `cmove %al, %al` → Ok(`[0f, 44, c0]`); also `cmove %xmm0, %eax` → Ok(`[0f, 44, c0]`); `cmove %ax, %eax` → Ok(`[0f, 44, c0]`)
**Expected:** `Err(...)` for all three classes
**Actual:** `Ok` with GP-aliased bytes
**Severity:** medium
**Root cause:** Both register numbers are taken solely from `reg_num`, which aliases r8/xmm/mm/st/ymm onto 0..7, with no `reg_size` or GP-class gate before encoding.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1119`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.bytes.push(self.modrm(3, dst_num, src_num));
```
**Suggested fix:** Gate on GP r16/r32 same-width (and reject xmm/mm/st/ymm prefixes) before `reg_num`:
```rust
                let src_sz = reg_size(&src.name);
                let dst_sz = reg_size(&dst.name);
                if src.name.starts_with("xmm") || src.name.starts_with("mm")
                    || src.name.starts_with("st") || src.name.starts_with("ymm")
                    || dst.name.starts_with("xmm") || dst.name.starts_with("mm")
                    || dst.name.starts_with("st") || dst.name.starts_with("ymm")
                {
                    return Err("cmovcc requires GP registers".into());
                }
                if src_sz != dst_sz || (src_sz != 2 && src_sz != 4) {
                    return Err("cmovcc requires matching r16/r32".into());
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cmovcc_regression_rejects_al_bl -- --test-threads=1 --nocapture
cargo test --lib test_encode_cmovcc_regression_rejects_xmm0 -- --test-threads=1 --nocapture
cargo test --lib test_encode_cmovcc_regression_rejects_ax_ebx -- --test-threads=1 --nocapture
```
**Raw output:**
```text
cmove %al, %bl must Err (r8 invalid); got Ok(Ok([15, 68, 216]))
SUT accepted non-GP `cmove %eax, %xmm0` → [0f, 44, c0]
SUT accepted mismatched-width `cmove %ax, %eax` → [0f, 44, c0]
minimal failing input: src = "al", dst = "al", m = "cmove"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs (test_encode_cmovcc_regression_rejects_al_bl, test_encode_cmovcc_regression_rejects_ax_ebx, test_encode_cmovcc_regression_rejects_xmm0)
