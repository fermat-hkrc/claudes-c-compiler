# Bug: encode_fcvt_precision encodes same-precision FCVT (unallocated)
**Law:** ARM ARM Floating-point data-processing (1 source) FCVT is unallocated when ftype==opc (dest precision equals source precision). llvm-mc rejects `fcvt s0, s1` / `fcvt d0, d1` / `fcvt h0, h1` as "invalid operand for instruction". Same-precision conversion must Err.
**Impact:** `fcvt s0, s0` (and D,D / H,H) is assembled as a 32-bit word instead of being rejected. The encoding is UNALLOCATED and would SIGILL on hardware.
**Function:** encode_fcvt_precision
**Detected by:** Negative/error contract — ARM ARM ftype==opc unallocated; llvm-mc "invalid operand"
**Minimal input:** `[Reg("s0"), Reg("s0")]` (shrunk `rd=0, rn=0, ty=0`; also D,D and H,H; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word(0x1e224000)) for s0,s0 — ftype=00 opc=00 (same precision)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_precision_regression_same_precision_s (also _d and _h)
