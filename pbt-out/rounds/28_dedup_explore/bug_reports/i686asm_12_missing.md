# i686 assembler lacks 12 system/legacy instructions (shrdl-reg, xlat, enter, into, bound, arpl, lldt/sldt/str/verr/verw/lar)
> **Filed as [issue #1039](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1039)**

# Bug: i686 assembler lacks 12 system/legacy instructions
**Law:** These are standard i386 instructions that GNU as and llvm-mc assemble.
**Impact:** External .s files for i686 using any of these fail. 12 GAPs (both refs accept, ccc rejects): `shrdl %eax, %ebx` (register-only shift), `xlat`, `enter $16, $0` (and nested `$1`), `into`, `bound %eax, (%ebx)`, `arpl %ax, (%ebx)`, `lldt %ax`, `sldt %ax`, `str %ax`, `verr %ax`, `verw %ax`, `lar %eax, %ebx`. 13/25 valid forms work (shldl, bswap, setz, cmps*, lgdt/sgdt, invlpg, wbinvd).
**Function:** i686 assembler mnemonic table
**Detected by:** round-28 i686 deep battery (25 forms; 13 OK, 12 GAPs)
**Minimal input:** `.text` / `enter $16, $0` → ccc rejects; both refs accept
**Severity:** low (completeness)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nenter $16, $0\n' > /tmp/t.s
./target/release/ccc -m32 -c -o /tmp/t.o /tmp/t.s  # error
as --32 /tmp/t.s -o /dev/null  # OK
```

