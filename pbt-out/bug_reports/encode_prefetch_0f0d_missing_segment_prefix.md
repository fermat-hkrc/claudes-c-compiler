# Bug: encode_prefetch_0f0d drops segment override prefix

**Law:** For `prefetchw` and any memory operand with a segment override, the encoded bytes must begin with the corresponding segment-prefix byte (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before `0F 0D /1`, matching Intel SDM / gas / llvm-mc and the sibling x86-64 `encode_sse_mem_only(ops, &[0x0F, 0x0D], 1)` path.
**Impact:** Any AT&T form such as `prefetchw %fs:(%eax)` or `prefetchw %gs:8(%ebx)` assembles without the segment override. The resulting machine code prefetches the wrong address space (default DS instead of FS/GS/…), which silently breaks TLS-relative and far-segment software prefetch-for-write sequences on i686.
**Function:** encode_prefetch_0f0d
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:25
**Detected by:** Differential — llvm-mc i686 (`-triple=i686 -show-encoding`)
**Minimal input:** `prefetchw` with `Memory { segment: Some("es"), base: %eax, disp: 0 }` (also FS/GS/CS/SS/DS; FS KAT witness)
**Expected:** `[0x26, 0x0f, 0x0d, 0x08]` for `%es:(%eax)`; `[0x64, 0x0f, 0x0d, 0x08]` for `%fs:(%eax)`
**Actual:** `[0x0f, 0x0d, 0x08]` (segment prefix omitted)
**Severity:** high
**Root cause:** `system.rs:30-32` emits `0F 0D` and calls `encode_modrm_mem` without first calling `emit_segment_prefix(mem)`, unlike sibling memory encoders in `gp_integer.rs` and x86-64 `encode_sse_mem_only`. Same defect class as `encode_prefetch` (0F 18).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:30`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x0D]);
                self.encode_modrm_mem(hint, mem)
            }
```
**Suggested fix:** Emit the segment override before the opcode, reusing the existing helper:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x0D]);
                self.encode_modrm_mem(hint, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_prefetch_0f0d_diff_segment_prefix -- --test-threads=1
cargo test --lib test_encode_prefetch_0f0d_regression_missing_fs_prefix -- --test-threads=1
cargo test --lib encode_prefetch_0f0d_kat_llvm_mc_segment_fs -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 13, 8]`,
 right: `[38, 15, 13, 8]`: segment prefix diff for `prefetchw %es:(%eax)`: SUT=[0f, 0d, 08] llvm-mc=[26, 0f, 0d, 08]
minimal failing input: seg = "es", base = "eax", disp = 0
	successes: 0
	local rejects: 0
	global rejects: 0

assertion `left == right` failed: SUT must emit FS override 0x64 before 0F 0D
  left: [15, 13, 8]
 right: [100, 15, 13, 8]

assertion `left == right` failed: prefetchw %fs:(%eax) must be [64, 0f, 0d, 08], got [0f, 0d, 08]
  left: [15, 13, 8]
 right: [100, 15, 13, 8]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs (`test_encode_prefetch_0f0d_regression_missing_fs_prefix`, `encode_prefetch_0f0d_kat_llvm_mc_segment_fs`)
