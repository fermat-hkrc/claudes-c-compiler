# Bug: ADD/SUB silently encodes ROR (and other invalid shifts/extends)
**Law:** AArch64 ADD/SUB shifted-register form allows only LSL/LSR/ASR with a shift in range (0..=63 if sf=1, 0..=31 if sf=0). ROR is not a valid ADD/SUB shift. llvm-mc and GNU as reject `add w0, w1, w2, ror #0`. Extended-register `imm3 > 4` is UNALLOCATED. The encoder must return Err, not a 32-bit word.
**Impact:** Invalid assembly is turned into a well-formed but wrong instruction (ROR is defaulted to LSL). Callers that pass a bad shift kind or out-of-range amount get silent mis-encoding instead of an assembler error.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w1"), Reg("w2"), Shift { kind: "ror", amount: 0 }], is_sub=false, set_flags=false
**Expected:** Err
**Actual:** Ok(Word) — unknown shift kinds take the `_ => 0b00` arm (LSL); shift amounts are masked with `& 0x3F`; extend amounts with `& 0x7`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_ror_rejected
**Serial reconfirmation:** reproduced with `cargo test --lib encode_add_sub_neg_invalid_shift_extend -- --test-threads=1`
