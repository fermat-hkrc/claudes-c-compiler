# x86-64 assembler lacks BMI1 extract/modify instructions (blsi/blsmsk/blsr + bextri) — 9 forms
> **Filed as [issue #1054](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1054)**

# Bug: x86-64 assembler lacks BMI1 extract/modify instructions
**Law:** blsi (extract lowest set isolated), blsmsk (get mask up to lowest set bit), blsr (reset lowest set bit) are BMI1 instructions. bextri (bit field extract immediate) is BMI2. GNU as assembles all.
**Impact:** 9 GAPs: blsi/blsmsk/blsr each in 3 forms (r32-r32, r64-r64, r64-mem) + bextri. These are fundamental for bit manipulation code (CPUs with BMI support them since Haswell/2013). 15/24 BMI forms work (andn/bextr/bzhi/pdep/pext/mulx/rorx/sarx/shlx/shrx all byte-exact ✓; tzcnt/lzcnt/adcx/adox on #1040).
**Function:** x86 assembler mnemonic table
**Detected by:** round-28 BMI battery (24 forms; 15 OK, 9 GAPs)
**Minimal input:** `.text` / `blsi %eax, %ebx` → ccc rejects; as accepts
**Severity:** low (completeness)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
blsi %%eax, %%ebx
' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error: unhandled
as /tmp/t.s -o /dev/null  # OK
```
