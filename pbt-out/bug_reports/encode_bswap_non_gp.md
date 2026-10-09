# Bug: encode_bswap accepts xmm/mm/st/ymm via reg_num aliasing

**Law:** BSWAP encodes only a GP r32; XMM/MMX/x87/YMM names must not produce `0F C8+rd` (Intel SDM Vol.2 BSWAP; llvm-mc rejects `bswapl %xmm0`).
**Impact:** A mistaken non-GP operand is silently turned into a GP BSWAP (e.g. `%xmm0` → same bytes as `%eax`), so wrong object code ships without an assembler error.
**Function:** encode_bswap
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:945
**Detected by:** Negative/Error Contract (non-GP) + differential cross-check vs llvm-mc
**Minimal input:** `bswapl %xmm0` (also mm0, st, st(0), ymm0, …)
**Expected:** `Err(...)` — not a GP r32
**Actual:** `Ok([0x0f, 0xc8])` — same bytes as `bswapl %eax`
**Severity:** medium
**Root cause:** `gp_integer.rs:951-953` uses bare `reg_num`, and `registers.rs:4-15` maps `xmm0|mm0|st|st(0)|ymm0` onto register number 0 with no class check in encode_bswap.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:950`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xC8 + num]);
                Ok(())
            }
```
**Suggested fix:** Reject non-GP names (and non-r32) before `reg_num`:
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 4
                    || is_xmm(&reg.name)
                    || is_mm(&reg.name)
                    || reg.name.starts_with("st")
                    || reg.name.starts_with("ymm")
                {
                    return Err("bswap requires 32-bit GP register".into());
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xC8 + num]);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bswap_neg_non_gp -- --test-threads=1
cargo test --lib test_encode_bswap_regression_rejects_xmm0 -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP `bswapl %xmm0` → [0f, c8]; BSWAP requires GP r32 (Intel SDM; reg_num must not alias xmm/mm/st/ymm).
minimal failing input: r = "xmm0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bswap_pbt.rs (test_encode_bswap_regression_rejects_xmm0)
