# x86-64 parser accepts xmm registers in GPR operand slots (addq/shlq/pushq/imulq silently encode as rax)
> **Filed as [issue #826](https://github.com/fermat-hkrc/claudes-c-compiler/issues/826)**

# Bug: x86-64 parser accepts xmm registers in GPR operand slots
**Law:** GPR-only instructions (add/shl/push/imul) must reject SSE register names in integer operand slots.
**Impact:** `addq %xmm0, %rax` encodes `4801c0` = add %rax,%rax (xmm0's numeric suffix parsed as reg 0); `shlq $3, %xmm0` → `48c1e003` = shl $3,%rax; `pushq %xmm0` → `50` = push %rax; `imulq %xmm0, %rax` → `480fafc0` = imul %rax,%rax. Every invalid xmm-in-GPR form silently becomes the rax version. Both as and llvm-mc reject all four.
**Function:** x86 assembler parser (register-name → register-class binding, the #825 class on the x86 path)
**Detected by:** round-26h xmm-in-GPR-slot battery (6 forms, 5 BUGs, all double-reference confirmed)
**Minimal input:** `.text` / `pushq %xmm0` → ccc assembles `50` (push %rax)
**Expected:** Err (register class mismatch)
**Actual:** silently encoded as rax
**Severity:** medium
**Regression test:** (to add to pbt_asm_deep)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\npushq %%xmm0\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s && readelf -x .text /tmp/t.o  # 50
as /tmp/t.s -o /dev/null  # Error: operand type mismatch
```

