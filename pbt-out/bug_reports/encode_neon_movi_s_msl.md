# Bug: encode_neon_movi encodes MSL as unshifted LSL #0
**Law:** ∀ rd ∈ [0,31], T ∈ {2s,4s}, imm8 ∈ [0,255], n ∈ {8,16}. encode_neon_movi([Vd.T, #imm8, msl #n]) = llvm-mc("movi Vd.T, #imm8, msl #n")
**Impact:** `movi v0.2s, #0, msl #8` is a valid GNU/ARM form (cmode=1100, each 32-bit lane = (imm8 << 8) | 0xFF). The SUT treats a non-`lsl` Shift as cmode=0000, so MSL #8 encodes as unshifted `#0`. Sibling encode_neon_mvni already implements MSL. Parser does not currently tokenize `msl` (parser.rs:1885), so the public assembler also cannot accept the form; the encoder still produces a silently wrong word for a well-formed Operand::Shift { kind: "msl" } list.
**Function:** encode_neon_movi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:624
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_movi([v0.2s, #0, msl #8])
**Expected:** Ok(Word(0x0f00c400)) — llvm-mc encoding of `movi v0.2s, #0, msl #8`
**Actual:** Ok(Word(0x0f000400)) — same as `movi v0.2s, #0` (cmode=0000)
**Severity:** medium
**Root cause:** neon.rs:678-687 only special-cases kind == "lsl"; any other Shift kind, including "msl", falls through to (cmode=0000, shift=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:678`
```rust
                    if kind == "lsl" {
                        match amount {
                            0 => (0b0000u32, 0),
                            8 => (0b0010, 8),
                            16 => (0b0100, 16),
                            24 => (0b0110, 24),
                            _ => return Err(format!("movi: unsupported shift amount: {}", amount)),
                        }
                    } else {
                        (0b0000, 0)
                    }
```
**Suggested fix:** Handle MSL #8/#16 as cmode 1100/1101 (as encode_neon_mvni does) and reject unknown shift kinds.
```rust
                    if kind == "lsl" {
                        match amount {
                            0 => (0b0000u32, 0),
                            8 => (0b0010, 8),
                            16 => (0b0100, 16),
                            24 => (0b0110, 24),
                            _ => return Err(format!("movi: unsupported shift amount: {}", amount)),
                        }
                    } else if kind == "msl" {
                        match amount {
                            8 => (0b1100u32, 8),
                            16 => (0b1101, 16),
                            _ => return Err(format!("movi: unsupported MSL shift: {}", amount)),
                        }
                    } else {
                        return Err(format!("movi: unsupported shift kind: {}", kind));
                    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_s_msl -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_movi_pbt::test_encode_neon_movi_regression_s_msl' (2277466) panicked at src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs:603:5:
assertion `left == right` failed: movi v0.2s, #0, msl #8 must match llvm-mc (cmode=1100, 0x0f00c400)
  left: 251659264
 right: 251708416
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
