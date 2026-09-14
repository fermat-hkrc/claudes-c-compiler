# Bug: encode_div accepts mixed x/w register widths
**Law:** ARM ARM UDIV/SDIV require same-width GPRs (`<Wd>, <Wn>, <Wm>` or `<Xd>, <Xn>, <Xm>`). llvm-mc rejects mixed x/w.
**Impact:** Assembler encodes `sdiv w0, w0, x0` using sf from Rd only (32-bit) and Rm's number, producing a well-formed 32-bit SDIV that does not match the written operands. Mixed-width typos assemble instead of erroring.
**Function:** encode_div
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_div([Reg("w0"), Reg("w0"), Reg("x0")], unsigned=false)` i.e. `sdiv w0, w0, x0` (rd64=false, rn64=false, rm64=true)
**Expected:** Err
**Actual:** Ok(Word(0x1ac00c00)) — encodes as 32-bit `sdiv w0, w0, w0` (sf from Rd; Rn/Rm widths unchecked). Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_div_pbt::test_encode_div_regression_mixed_width
