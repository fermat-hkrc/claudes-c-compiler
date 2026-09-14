# Bug: encode_shift omits FS/GS segment override on memory operands
**Law:** A memory operand with an FS or GS segment override must emit prefix 0x64 or 0x65 before the shift opcode, matching GAS/llvm-mc AT&T encoding.
**Impact:** `shlb $0, %fs:(%rax)` (and any FS/GS memory shift/rotate) assembles without the segment prefix, so the instruction reads/writes the default DS segment instead of FS/GS. TLS and per-CPU accesses that use `%fs:`/`%gs:` silently target the wrong address.
**Function:** encode_shift
**Detected by:** Differential — llvm-mc -triple=x86_64 -show-encoding
**Minimal input:** mnemonic=shlb, shift_op=4, ops=[Imm(0), Mem(segment=fs, base=rax, disp=none)], AT&T `shlb $0, %fs:(%rax)`
**Expected:** bytes `[0x64, 0xc0, 0x20, 0x00]` (llvm-mc)
**Actual:** bytes `[0xc0, 0x20, 0x00]` — the 0x64 FS prefix is missing
**Severity:** high
**Regression test:** src/backend/x86/assembler/encoder/gp_integer.rs `test_encode_shift_regression_fs_segment_prefix`
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced the same witness.
**Root cause:** encode_shift never calls `emit_segment_prefix`. core.rs:47-48 documents that other instruction families accepting memory operands should emit the override (currently only mov/ALU/push/pop do).
