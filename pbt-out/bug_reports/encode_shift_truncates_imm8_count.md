# Bug: encode_shift truncates shift counts outside imm8
**Law:** The shift/rotate immediate is an imm8. Counts that do not fit in 8 bits (llvm-mc rejects `$256`) must be rejected, not silently truncated.
**Impact:** `shlq $256, %rax` encodes as `shlq $0, %rax` (`count as u8` wraps 256 → 0). A count of 257 wraps to 1 and takes the D1 shift-by-1 form. Wrong shift amount in generated code.
**Function:** encode_shift
**Detected by:** Negative/Error Contract (4e) — Intel SDM Group 2 imm8; llvm-mc rejects `$256`
**Minimal input:** mnemonic=shlb, shift_op=4, ops=[Imm(256), Reg(al)], AT&T `shlb $256, %al`
**Expected:** Err
**Actual:** Ok — 256 is truncated to 0 and encoded as C0/C1 with imm8=0. Deterministic witness `shlq $256, %rax` → Ok([0x48, 0xc1, 0xe0, 0x00])
**Severity:** medium
**Regression test:** src/backend/x86/assembler/encoder/gp_integer.rs `test_encode_shift_regression_imm8_overflow_256`
**Serial reconfirm:** deterministic regression; property shrinks to count=256 independently of threads.
**Root cause:** `let count = *count as u8;` in both the register and memory Imm arms, with a subsequent `count == 1` check on the truncated value.
