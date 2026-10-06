# x86-64 SIB encoding silently drops %rsp as index register — 8(base,rsp,scale) encodes as [base+disp] with no index
> **Filed as [issue #834](https://github.com/fermat-hkrc/claudes-c-compiler/issues/834)**

# Bug: x86-64 SIB encoding silently drops %rsp as index register
**Law:** In x86 SIB encoding, index=100 (RSP) means "no index register". An explicit `(%base,%rsp,scale)` operand is therefore unencodable and both GNU as and llvm-mc reject it. The assembler must error, not silently emit the no-index form.
**Impact:** `movq 8(%rax,%rsp,4), %rdx` encodes `488b54a0 08` — SIB byte `a0` = scale=4·base=000·**index=100(no-index)** — the RSP index term is silently DROPPED. The instruction loads from `[rax+8*1]` (just `8(%rax)`) instead of `[rax+rsp*4+8]`. 160/1280 SIB matrix forms affected (every base × every scale × every displacement, all with %rsp as index).
**Function:** x86 assembler SIB byte encoding (index=4/RSP not rejected)
**Detected by:** round-26q 1280-form SIB full matrix (8 bases × 8 indexes × 4 scales × 5 displacements; 1120 clean byte-exact ✓; 160 BUGs — every %rsp-index combination)
**Minimal input:** `.text` / `movq 8(%rax,%rsp,4), %rdx` → ccc assembles `488b54a0 08`; as rejects
**Expected:** Err (rsp as index is unencodable in SIB)
**Actual:** silently encoded as [base+disp] (index term dropped)
**Severity:** medium (silent wrong address — the #776 class on the SIB path)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nmovq 8(%%rax,%%rsp,4), %%rdx\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # accepts! → 488b54a008
as /tmp/t.s -o /dev/null  # Error: bad expression / operand mismatch
```

