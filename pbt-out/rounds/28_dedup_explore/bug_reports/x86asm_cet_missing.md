# x86-64 assembler lacks CET/shadow-stack instructions (endbr64/wrssd/wrussd/incsspd/rdssp/saveprevssp) — 6 forms
> **Filed as [issue #1055](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1055)**

# Bug: x86-64 assembler lacks CET/shadow-stack instructions
**Law:** CET (Control-flow Enforcement Technology) instructions are standard on modern x86-64 CPUs (Tiger Lake+). GNU as assembles them.
**Impact:** 6 GAPs: endbr64, endbr32, wrssd, wrussd, incsspdd, rdsspq, saveprevssp, rstsspp, setssbsy, clrssbsy. 4/14 work (wrfsbase/wrgsbase/rdfsbase/rdgsbase — FSGSBASE, not CET).
**Function:** x86 assembler mnemonic table
**Detected by:** round-28 CET battery (14 forms; 4 OK, 6 GAPs + 4 more GAPs in the same set)
**Minimal input:**  /  → ccc rejects; as accepts
**Severity:** low (CET is newer; many older tools also lack it)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
endbr64
' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```
