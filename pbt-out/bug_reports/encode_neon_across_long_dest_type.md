# Bug: encode_neon_across_long ignores destination register type
**Law:** ARM ARM SADDLV/UADDLV destination `<V><d>` is a scalar SIMD register whose type matches T: H for 8B/16B, S for 4H/8H, D for 4S. Dest must not be B/Q/GPR/vector-arrangement.
**Impact:** `saddlv b0, v0.8b` (and s/d/q/x/w/v dest, or `v0.8b` as dest) is encoded as if the dest were `h0`. The assembler accepts text that gas/llvm-mc reject and can emit a word whose Rd number is taken from a register of the wrong class.
**Function:** encode_neon_across_long
**Detected by:** Negative/Error Contract (5)
**Minimal input:** rd=0, rn=0, u=0, t="8b", prefix="b", as_arr=false — `saddlv b0, v0.8b` (operands `[Reg("b0"), RegArrangement{v0, "8b"}]`, u=0, opcode=0b00011)
**Expected:** Err
**Actual:** Ok(Word) with Rd=0, same encoding as `saddlv h0, v0.8b`. Root cause: dest match extracts only `parse_reg_num` and ignores the prefix; `Operand::RegArrangement` dest is also accepted with its arrangement discarded.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_across_long_pbt::test_encode_neon_across_long_regression_dest_type
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_across_long_neg -- --test-threads=1`
