# same-TU inline + non-inline function definition accepted — gcc/clang reject (C99 6.7.4p7 constraint violation); the inline body silently wins (wrong definition selected)
> **Filed as [issue #1133](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1133)**

# Bug: same-TU inline + non-inline function definition accepted — gcc/clang reject (C99 6.7.4p7 constraint violation); the inline body silently wins (wrong definition selected)
**Law:** ∀ function f declared both `inline` and non-inline (external definition) in the same translation unit: the compiler must emit a redefinition diagnostic (C99 6.7.4p7 + 6.7p3 — the inline definition and the external definition cannot coexist in one TU; exactly one TU may provide the external definition separately). FALSIFIED — ccc accepts and silently uses the inline body.
**Impact:** Silent wrong-code at source level: the author's external definition (the C99 idiom for providing the out-of-line copy) is discarded; the inline body wins. gcc and clang both reject:
```
$ gcc -w t.c   → error: redefinition of 'f'
$ clang -w t.c → error: redefinition of 'f'
$ ccc t.c -o t && ./t
f=1              ← inline body (return 1) silently chosen; external body (return 2) dead
```
**Function:** parser/sema definition-merging — inline-qualified and unqualified definitions of the same symbol are not recognized as a redefinition conflict (src/frontend/sema or parser symbol handling).
**Detected by:** round-40 probe battery (C99 inline-across-TU semantics target).
**Minimal input:**
```c
inline int f(void) { return 1; }
int f(void) { return 2; }
int main(void) { return f(); }   // ccc: compiles, returns 1; gcc/clang: redefinition error
```
**Expected:** diagnostic "redefinition of 'f'" (both references reject).
**Actual:** compiles clean; the inline definition is selected at the call site.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'inline int f(void){return 1;}\nint f(void){return 2;}\n#include <stdio.h>\nint main(){printf("f=%%d\\n", f());}' > /tmp/r.c
gcc -w /tmp/r.c -o /tmp/g 2>&1 | head -1   # error: redefinition
./target/debug/ccc /tmp/r.c -o /tmp/c && /tmp/c   # f=1
```
