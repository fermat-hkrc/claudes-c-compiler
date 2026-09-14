# Bug: encode_uxtw emits MOV Wd,Wn (ORR) instead of UBFM Xd,Xn,#0,#31
**Law:** ARM ARM C6 UXTW is the assembler alias of UBFM Xd, Xn, #0, #31 (sf=1 opc=10 N=1 immr=0 imms=31). llvm-mc `-triple=aarch64` encodes `uxtw x0, w1` as `0xD3407C20` (disassembled `ubfx x0, x1, #0, #32`). GNU-style assembler contract: same textual assembly as gas, same 32-bit word.
**Impact:** Every `uxtw` instruction is assembled as 32-bit `ORR Wd, WZR, Wn` (`MOV Wd, Wn`). Object files disagree with gas/llvm-mc; disassembly shows `mov w0, w1` instead of `ubfx x0, x1, #0, #32`. Semantically similar for the register file but a wrong encoding for an assembler.
**Function:** encode_uxtw
**Detected by:** Differential vs llvm-mc; Algebraic — Metamorphic (UBFM #0,#31 alias); Algebraic — Invariant (ARM fields)
**Minimal input:** `[Reg("x0"), Reg("w1")]` (uxtw x0, w1); also shrunk `[Reg("x0"), Reg("w0")]` (rd=0, rn=0)
**Expected:** `Ok(Word(0xD3407C20))` for x0, w1; `Ok(Word(0xD3407C00))` for x0, w0
**Actual:** `Ok(Word(0x2A0103E0))` for x0, w1 (`mov w0, w1`); `Ok(Word(0x2A0003E0))` for x0, w0. Formula used: `(0b001010100 << 23) | (rn << 16) | (0b11111 << 5) | rd` (32-bit ORR with Rn=WZR, Rm=Wn).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_uxtw_regression_mov_not_ubfm`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_uxtw -- --test-threads=1`
