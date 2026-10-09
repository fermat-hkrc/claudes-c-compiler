# Bug: encode_bswap accepts r16/r8 as if r32

**Law:** BSWAP on IA-32 is defined only for 32-bit general-purpose registers; encoding a 16-bit or 8-bit register name must be rejected (Intel SDM Vol.2 BSWAP; llvm-mc `-triple=i686` rejects `bswapl %ax` / `bswapl %al`).
**Impact:** An assembler consumer that feeds AT&T `bswapw`-style or mismatched-width operands gets a silent 32-bit BSWAP encoding (`0F C8+rd`) instead of an error, producing wrong machine code.
**Function:** encode_bswap
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:945
**Detected by:** Negative/Error Contract (wrong-width GP) + differential cross-check vs llvm-mc
**Minimal input:** `bswapl %ax` (also `%al`, any r16/r8; mnemonic `bswap` same)
**Expected:** `Err(...)` — operand not r32
**Actual:** `Ok([0x0f, 0xc8])` — same bytes as `bswapl %eax`
**Severity:** medium
**Root cause:** `gp_integer.rs:951-953` calls `reg_num` with no `reg_size == 4` gate; `reg_num` aliases ax/al onto the same 3-bit number as eax.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:950`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xC8 + num]);
                Ok(())
            }
```
**Suggested fix:** Require GP r32 before encoding:
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
cargo test --lib encode_bswap_neg_wrong_width -- --test-threads=1
cargo test --lib test_encode_bswap_regression_rejects_ax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid-width `bswapl %ax` → [0f, c8]; BSWAP is r32-only (Intel SDM; llvm-mc rejects).
minimal failing input: r = "ax", m = "bswapl"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bswap_pbt.rs (test_encode_bswap_regression_rejects_ax, test_encode_bswap_regression_rejects_al)
