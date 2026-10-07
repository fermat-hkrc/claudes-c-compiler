# x86-64 assembler lacks lzcnt/tzcnt (ABM) and adcx/adox (ADX) — 4 BMI-adjacent instructions missing
> **Filed as [issue #1040](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1040)**

# Bug: x86-64 assembler lacks lzcnt/tzcnt and adcx/adox instructions
**Law:** lzcnt/tzcnt are AMD ABM (adopted by Intel); adcx/adox are ADX extensions. GNU as and llvm-mc assemble all four.
**Impact:** External .s files using these fail to assemble. 4 GAPs: `lzcnt %eax, %ebx`, `tzcnt %r8, %r9`, `adcx %eax, %ebx`, `adox %eax, %ebx`. Note: `lzcnt/tzcnt` ARE supported via the `__builtin_clz/ctz` intrinsic path (E2E tested), but the raw mnemonic in .s files is missing. 26/30 other forms work (rep prefixes, cr/dr moves, cmov, BMI2: andn/bextr/bzhi/pdep/pext/rorx/sarx/shlx/shrx/mulx all byte-exact ✓).
**Function:** x86 assembler mnemonic table
**Detected by:** round-28 x86 deep battery (30 forms; 26 OK, 4 GAPs)
**Minimal input:** `.text` / `lzcnt %eax, %ebx` → ccc rejects; both refs accept
**Severity:** low (completeness)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nlzcnt %%eax, %%ebx\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```

