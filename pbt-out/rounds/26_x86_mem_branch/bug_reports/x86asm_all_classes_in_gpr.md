# x86 parser accepts every non-GPR register class in GPR slots (mm0/cs/st(0) in addq; cs in pushq) — #826's class extends to all register classes + i686 backend
> **Filed as [issue #827](https://github.com/fermat-hkrc/claudes-c-compiler/issues/827)**

# Bug: x86 parser accepts every non-GPR register class in GPR operand slots
**Law:** GPR-only instructions must reject every non-GPR register name (MMX, segment, x87 stack) in integer operand slots. The numeric suffix of any register name (mm0→0, cs→1, st(0)→0) must not be silently reinterpreted.
**Impact:** `addq %mm0, %rax` encodes `4801c0` = add %rax,%rax; `addq %cs, %rax` → `4801c8` = add %rax,%rcx (cs's internal num 1); `pushq %cs` → `51` = push %rcx; `addq %st(0), %rax` → `4801c0` = add %rax,%rax. i686 same class: `addl %xmm0, %eax` → `01c0`; `pushl %xmm0` → `50` = push %eax. Both as and llvm-mc reject all six.
**Function:** x86 assembler parser (register-name → register-class binding — NO class check at all; #826 was the xmm case, this extends it to mm0/cs/st(0) + the i686 path)
**Detected by:** round-26i cross-class battery (12 forms, 6 new BUGs across x86-64 and i686)
**Minimal input:** `.text` / `addq %cs, %rax` → ccc assembles `4801c8` (add %rax,%rcx)
**Expected:** Err (register class mismatch)
**Actual:** silently encoded as the internal-number GPR
**Severity:** medium
**Regression test:** (to add to pbt_asm_deep)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\naddq %%cs, %%rax\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s && readelf -x .text /tmp/t.o  # 4801c8
as /tmp/t.s -o /dev/null  # Error
```

