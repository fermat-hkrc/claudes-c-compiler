# x86-64 assembler rejects cmpxchg register-to-register form (cmpxchg %eax, %ecx)
> **Filed as [issue #840](https://github.com/fermat-hkrc/claudes-c-compiler/issues/840)**

# Bug: x86-64 assembler rejects cmpxchg register-to-register form
**Law:** cmpxchg r/m32, r32 has a register-destination form; GNU as and llvm-mc assemble it.
**Impact:** `cmpxchg %eax, %ecx` fails to assemble. The memory form (`cmpxchgq %rax, (%rbx)`) works. External .s files using the reg-reg CAS idiom break.
**Function:** x86 assembler cmpxchg encoder (memory-only path)
**Detected by:** round-26w 22-form bit-test/cmpxchg battery (21 work; this GAP)
**Minimal input:** `.text` / `cmpxchg %eax, %ecx` → ccc rejects; as accepts
**Expected:** assembles (0F B1 /r with reg-reg ModRM)
**Actual:** rejected
**Severity:** low (completeness)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\ncmpxchg %%eax, %%ecx\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```

