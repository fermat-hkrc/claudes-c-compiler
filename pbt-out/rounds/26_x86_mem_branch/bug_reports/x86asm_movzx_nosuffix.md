# x86-64 assembler rejects suffix-less movzx/movsx forms and cwde/cdqe — 8 valid mnemonics missing
> **Filed as [issue #841](https://github.com/fermat-hkrc/claudes-c-compiler/issues/841)**

# Bug: x86-64 assembler rejects suffix-less movzx/movsx and cwde/cdqe
**Law:** AT&T syntax allows suffix-less movzx/movsx when the width is inferable from the destination register (GNU as and llvm-mc both accept); cwde/cdqe are standard sign-extend-into-eax/rax mnemonics.
**Impact:** External .s files using the shorter no-suffix forms (`movzx %al, %rax` instead of `movzbq %al, %rax`) or the cdqe instruction (sign-extend eax to rax, the standard `i32→i64` idiom) fail to assemble. 8 affected: movzx ×3 (al→rax, al→eax, ax→eax), movsx ×3 (same), cwde (cbw 32-bit form), cdqe (cbw 64-bit form). The suffixed forms (movzbq/movsbq/etc.) and cdq/cqto all work byte-exact (16/16 verified).
**Function:** x86 assembler mnemonic table (no-suffix inference + cwde/cdqe entries)
**Detected by:** round-26y 24-form sign/zero-extend battery (16 byte-exact ✓; 8 GAPs)
**Minimal input:** `.text` / `movzx %al, %eax` → ccc rejects; as accepts (assembles as movzbl)
**Expected:** assembles (width inferred from destination register)
**Actual:** rejected
**Severity:** low (completeness; suffixed equivalents work)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nmovzx %%al, %%eax\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```

