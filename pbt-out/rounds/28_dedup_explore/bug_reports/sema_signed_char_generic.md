# sema conflates signed char with char in _Generic type matching — C11 requires three distinct char types
> **Filed as [issue #1047](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1047)**

# Bug: sema conflates signed char with char in _Generic type matching
**Law:** C11 6.2.5p15: char, signed char, and unsigned char are three DISTINCT types.
**Impact:** _Generic(sc, char: 0, signed char: 1) selects 0 instead of 1. Generic dispatch goes to wrong branch.
**Function:** sema type comparison for _Generic
**Detected by:** round-28 _Generic 20-type matrix (19/20 match; signed char conflated with char)
**Minimal input:** signed char sc = -1; _Generic(sc, char: 0, signed char: 1) → gcc: 1, ccc: 0
**Severity:** medium
**Contract evidence:** inferred (C11 6.2.5p15; gcc distinguishes)
**Regression test:** (deterministic: must return 1)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '#include <stdio.h>
int main(void){signed char sc=-1;printf(\%%s\n\,_Generic(sc,char:\char\,signed char:\schar\,default:\other\));return 0;}
' > /tmp/t.c
gcc -w -std=c11 /tmp/t.c -o /tmp/t && /tmp/t   # schar
./target/release/ccc /tmp/t.c -o /tmp/t2 && /tmp/t2  # char (WRONG)
```
