# Bug: encode_neon_qshrn accepts shift above destination element size
**Law:** SQSHRN/UQSHRN/SQRSHRN/UQRSHRN shift must be an integer in range [1, dest element size] (8 for Ta=8H, 16 for Ta=4S, 32 for Ta=2D). Inputs outside that range must be rejected.
**Impact:** A shift of dest_esize+1 through source_esize encodes a word whose immh:immb does not match the source arrangement: for `sqshrn v0.8b, v0.8h, #9` the SUT emits immh=0000, which ARM ARM classifies as a different encoding group (not a shift-by-immediate). Assembled output is silently wrong rather than an assembler error.
**Function:** encode_neon_qshrn
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, ta=8h, tb=8b, shift=9, u_bit=0, is_rounding=false, is_high=false (`sqshrn v0.8b, v0.8h, #9`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [1, 8]")
**Actual:** Ok(Word) — body checks `shift == 0 || shift > element_bits` with element_bits = source size 16, so 9 is accepted; immhb = 16-9 = 7 (immh=0000)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_qshrn_regression_shift_oob_dest_esize
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_qshrn_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law (not the producing statement):
- llvm-mc `-triple=aarch64`: `sqshrn v0.8b, v1.8h, #9` → "immediate must be an integer in range [1, 8]"
- ARM ARM Advanced SIMD shift by immediate: `esize = 8 << HighestSetBit(immh)` is dest element size; `shift = (2*esize) - UInt(immh:immb)`; `immh != 0000`
- Sibling `encode_neon_shrn` at neon.rs:1443-1444: `half_bits = element_bits / 2; if shift == 0 || shift > half_bits`
