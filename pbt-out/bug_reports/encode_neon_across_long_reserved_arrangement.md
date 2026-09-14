# Bug: encode_neon_across_long encodes reserved SADDLV arrangements (2S/1D/2D)
**Law:** ARM ARM SADDLV/UADDLV source arrangement T is only 8B, 16B, 4H, 8H, or 4S. size=11 (1D/2D) and size=10 with Q=0 (2S) are reserved/UNDEFINED and must be rejected.
**Impact:** `saddlv h0, v0.2s` (and 1d/2d) is encoded as an UNDEFINED instruction instead of an assembler error. llvm-mc/gas reject these operands. A later decoder or CPU may treat the word as UNALLOCATED.
**Function:** encode_neon_across_long
**Detected by:** Negative/Error Contract (5)
**Minimal input:** rd=0, rn=0, u=0, t="2s" — `saddlv h0, v0.2s` (operands `[Reg("h0"), RegArrangement{v0, "2s"}]`, u=0, opcode=0b00011)
**Expected:** Err
**Actual:** Ok(Word) with Q=0, size=10 (the reserved 2S encoding). Root cause: `neon_arr_to_q_size` maps "2s"/"1d"/"2d" and encode_neon_across_long does not filter those reserved T values.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_across_long_pbt::test_encode_neon_across_long_regression_reserved_2s
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_across_long_neg -- --test-threads=1`
