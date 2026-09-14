# Bug: encode_neon_shll accepts out-of-range shift (and panics on Imm(-1))
**Law:** SSHLL/USHLL shift must be an integer in range [0, esize-1] (7 for 8-bit Tb, 15 for 16-bit, 31 for 32-bit). Inputs outside that range must be rejected.
**Impact:** Two related failures on inputs the public helper accepts:
1. `sshll v0.8h, v0.8b, #8` (bound+1) encodes immhb = 8+8 = 16, which ARM ARM reads as a 16-bit source with shift 0 — a different instruction size, silently wrong rather than an assembler error.
2. `sshll v0.8h, v0.8b, #-1` does `get_imm as u32` (u32::MAX) then `base_val + shift`, which panics with "attempt to add with overflow" in debug and wraps in release.
**Function:** encode_neon_shll
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, tb=8b, shift=-1, u_bit=0, is_high=false (shrunk). Related bound+1: shift=8 (`sshll v0.8h, v0.8b, #8`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 7]")
**Actual:** debug panic on #-1; Ok(Word) with wrong immh on #8 — body has no shift-range check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_shll_regression_shift_neg and test_encode_neon_shll_regression_shift_oob
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_shll_neg -- --test-threads=1` reproduced the failure (overflow panic on the shrunk #-1 witness).

Doc evidence for the law (not the producing statement):
- llvm-mc `-triple=aarch64`: `sshll v0.8h, v1.8b, #8` → "immediate must be an integer in range [0, 7]"
- ARM ARM Advanced SIMD SSHLL: shift in 0 to (esize-1); immh:immb = esize + shift; immh != 0000
