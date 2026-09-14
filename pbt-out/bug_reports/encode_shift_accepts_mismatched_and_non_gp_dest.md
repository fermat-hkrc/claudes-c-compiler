# Bug: encode_shift accepts size-mismatched and non-GP destinations
**Law:** A shift/rotate destination must be a general-purpose register (or memory) whose width matches the mnemonic suffix. llvm-mc and Intel SDM Group 2 reject `shlw %al` and `shlq %xmm0`.
**Impact:** Public AT&T text such as `shlw $1, %al` and `shlq $1, %xmm0` is accepted and encoded as a different instruction (`shlw %ax` and `shlq %rax`), because `reg_num` maps `al`/`xmm0` to encoding 0. Silent wrong machine code.
**Function:** encode_shift
**Detected by:** Negative/Error Contract (4e) — llvm-mc/Intel SDM reject; README claims AT&T/GAS compatibility
**Minimal input:**
- `shlw $1, %al` (mnemonic=shlw, shift_op=4, ops=[Imm(1), Reg(al)])
- `shlq $1, %xmm0` (mnemonic=shlq, shift_op=4, ops=[Imm(1), Reg(xmm0)])
**Expected:** Err
**Actual:**
- `shlw $1, %al` → Ok([0x66, 0xd1, 0xe0]) which is `shlw %ax`
- `shlq $1, %xmm0` → Ok([0x48, 0xd1, 0xe0]) which is `shlq %rax`
**Severity:** high
**Regression test:** src/backend/x86/assembler/encoder/gp_integer.rs `test_encode_shift_regression_mixed_size_shlw_al` and `test_encode_shift_regression_non_gp_xmm0`
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced `shlw`/`al`; the xmm0 case is a deterministic regression.
**Root cause:** encode_shift uses `mnemonic_size_suffix` for opcode width and `reg_num` for ModR/M, with no check that the destination is a GP register of that width. `reg_num` aliases xmm/ymm/mm/st/segment names onto GP encodings.
