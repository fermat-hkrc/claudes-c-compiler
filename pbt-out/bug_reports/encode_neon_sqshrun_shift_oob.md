# Bug: SQSHRUN accepts shift amounts above dest element size
**Law:** GNU-style `sqshrun`/`sqrshrun`(+2) shift must be in 1..=dest_esize (8 for 8H→8B/16B, 16 for 4S→4H/8H, 32 for 2D→2S/4S). Out-of-range shift must be rejected.
**Impact:** An assembly line such as `sqshrun v0.8b, v0.8h, #9` is accepted and encoded as a *different* legal instruction (shift #1), so the object file silently contains the wrong shift.
**Function:** encode_neon_sqshrun
**Detected by:** Negative/Error Contract (5) — llvm-mc rejects the same text; ARM ARM shift = (2*esize) − UInt(immh:immb) with 1..=esize; sibling encode_neon_shrn uses half_bits = source/2
**Minimal input:** `sqshrun v0.8b, v0.8h, #9` — operands `[RegArrangement(v0, 8b), RegArrangement(v0, 8h), Imm(9)]`, is_rounding=false, is_high=false
**Expected:** Err (llvm-mc: error at the immediate; ARM range for 8H source is [1, 8])
**Actual:** Ok(Word). Body checks `shift == 0 || shift > element_bits` with element_bits = *source* size 16, so 9..=16 are accepted. For #9: immhb = 7, immh = (7>>3)|0b0001 = 1, immb = 7 encodes as shift #1.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_sqshrun_regression_shift_oob_dest_esize
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_neon_sqshrun -- --test-threads=1)
