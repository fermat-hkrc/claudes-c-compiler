# Bug: encode_invlpg drops segment override prefix

**Law:** For any valid memory operand carrying a segment override (es/cs/ss/ds/fs/gs), `encode_invlpg` must emit the corresponding segment-override prefix byte (0x26/0x2E/0x36/0x3E/0x64/0x65) before the INVLPG opcode bytes `0F 01 /7`, matching llvm-mc / Intel SDM.
**Impact:** Kernel/hypervisor code that invalidates a TLB entry via a non-default segment (commonly `%fs:` / `%gs:`) assembles to the wrong address space: the instruction still runs as INVLPG but against DS-relative memory, silently missing the intended page and leaving stale TLB entries.
**Function:** encode_invlpg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:110
**Detected by:** Differential — llvm-mc i686 (encode_invlpg_diff_llvm_mc_segment / KAT fs)
**Minimal input:** `invlpg %es:(%eax)` (also `%fs:(%eax)`, `%gs:8(%ebx)`, SIB forms)
**Expected:** `[0x26, 0x0f, 0x01, 0x38]` for `%es:(%eax)`; `[0x64, 0x0f, 0x01, 0x38]` for `%fs:(%eax)`
**Actual:** `[0x0f, 0x01, 0x38]` (segment prefix omitted)
**Severity:** high
**Root cause:** `system.rs:116-117` pushes `0F 01` and calls `encode_modrm_mem` without first calling `emit_segment_prefix(mem)`. `encode_modrm_mem` itself never emits segment prefixes; gp_integer callers do call `emit_segment_prefix`, and the x86-64 sibling `encode_mem_only` goes through `emit_rex_rm` which covers segment overrides. The i686 INVLPG path simply never wired that helper.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:114`
```rust
        match &ops[0] {
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(7, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode bytes.
```rust
        match &ops[0] {
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(7, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_invlpg_kat_llvm_mc_segment_fs -- --test-threads=1
cargo test --lib test_encode_invlpg_regression_missing_fs_prefix -- --test-threads=1
cargo test --lib encode_invlpg_diff_llvm_mc_segment -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: SUT must emit FS override 0x64 before 0F 01
  left: [15, 1, 56]
 right: [100, 15, 1, 56]

proptest minimal failing input: seg = "es", base = "eax", disp = 0
  left: [15, 1, 56]
 right: [38, 15, 1, 56]: segment prefix diff for `invlpg %es:(%eax)`
cc 3e05cfec304bbc270192f0cfec41758c0b2f36d6645bebfcb7548354393d2a3f
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs (`test_encode_invlpg_regression_missing_fs_prefix`, `test_encode_invlpg_regression_missing_gs_prefix_disp`)
