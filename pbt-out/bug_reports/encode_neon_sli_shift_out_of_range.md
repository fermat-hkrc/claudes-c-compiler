# Bug: encode_neon_sli does not reject out-of-range shift
**Law:** Vector SLI shift must be an integer in [0, esize(T)-1] (8B/16B: 0..7, 4H/8H: 0..15, 2S/4S: 0..31, 2D: 0..63). Values outside that range must be rejected with Err, matching llvm-mc `immediate must be an integer in range [0, esize-1]`.
**Impact:** Negative immediates panic in debug (`attempt to add with overflow` on `esize + shift`) and wrap in release, encoding a different legal shift. `shift == esize` is masked (`(esize + shift) & mask`) into reserved immh=0000, producing an undefined encoding that gas/llvm-mc reject.
**Function:** encode_neon_sli
**Detected by:** Negative/Error Contract (4e)
**Minimal input:**
- rd=0, rn=0, t="8b", shift=-1 — `sli v0.8b, v0.8b, #-1` panics in debug (`8 + ( -1 as u32 )` overflows)
- rd=0, rn=0, t="8b", shift=8 — `sli v0.8b, v0.8b, #8` returns Ok with immh:immb = (8+8)&0xF = 0 (reserved)
**Expected:** Err
**Actual:** debug panic on shift < 0; Ok(Word) with wrapped/masked immh:immb on shift >= esize
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_sli_pbt::test_encode_neon_sli_regression_negative_shift and test_encode_neon_sli_regression_shift_eq_esize
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_sli_neg -- --test-threads=1`
