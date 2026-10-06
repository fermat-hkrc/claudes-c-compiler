# Bug: encode_ldnp_stnp encodes SIMD S/D/Q pairs as integer (V=0)
**Law:** LDNP/STNP of S/D/Q registers must set V=1 and opc=00/01/10 with scale 4/8/16, matching llvm-mc/gas and ARM ARM C6 LDNP (SIMD&FP)
**Impact:** `ldnp d0, d1, [x2, #16]` (and S/Q variants) is assembled as a 32-bit integer pair with the wrong opc/V/imm7, so FP spill/fill is the wrong instruction
**Function:** encode_ldnp_stnp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:518
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_ldnp_stnp([Reg("s0"), Reg("s0"), Mem{base:"x0", offset:-256}], is_load=false)
**Expected:** llvm-mc `stnp s0, s0, [x0, #-256]` = 0x2C1F8000 (V=1)
**Actual:** Ok(Word(0x281F8000)) — integer STNP W0, W0, [X0, #-256] (V=0)
**Severity:** medium (documented by the author)
**Root cause:** load_store.rs:517 TODO admits V=1 is unimplemented; the body never calls is_fp_reg and hardcodes V=0, so s/d/q names go through get_reg as 32-bit integer (is_64bit_reg is false for d/s/q)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:535`
```rust
            let word = (opc << 30) | (0b101 << 27) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
```
**Suggested fix:** Detect FP/SIMD Rt (s/d/q), set V=1, opc and scale from the prefix, matching encode_ldp_stp
```rust
    let fp = is_fp_reg(rt1_name);
    let v = if fp { 1u32 } else { 0u32 };
    // opc/shift from s/d/q as in encode_ldp_stp; OR V into the word
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_simd -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `673185792`,
 right: `740294656`: SIMD mismatch for stnp s0, s0, [x0, #-256]
minimal failing input: is_load = false, kind = 0, rt1 = 0, rt2 = 0, rn = 0, imm7 = -64
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
