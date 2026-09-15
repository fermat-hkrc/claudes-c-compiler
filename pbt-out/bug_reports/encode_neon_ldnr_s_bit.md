# Bug: LD2R/LD4R S bit encoded at bit 12 instead of bit 21
**Law:** Valid ld2r/ld4r must match llvm-mc/gas: AdvSIMD replicate encoding places the structure-count S bit at bit 21 (1 for LD2R/LD4R, 0 for LD1R/LD3R) with bit 12 clear.
**Impact:** Every LD2R and LD4R instruction is assembled to the wrong 32-bit word, so objects disagree with gas/llvm-mc and will not execute the intended replicate load.
**Function:** encode_neon_ldnr
**Detected by:** Differential vs llvm-mc / algebraic.invariant (ARM fields)
**Minimal input:** `ld2r {v0.8b, v1.8b}, [x1]` → SUT 0x0d40d020, llvm-mc/gas 0x0d60c020. Also `ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x0]` → SUT 0x0d40f000, llvm-mc 0x0d60e000.
**Expected:** bit21=1, bit12=0 for n=2/4 (gas `ld2r {v0.8b, v1.8b}, [x1]` = 0x0d60c020)
**Actual:** bit21=0, bit12=1 (`s_bit << 12` in neon.rs:1569). LD3R happens to match because s_bit=0.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_ld2r_s_bit / test_encode_neon_ldnr_regression_ld4r_s_bit
