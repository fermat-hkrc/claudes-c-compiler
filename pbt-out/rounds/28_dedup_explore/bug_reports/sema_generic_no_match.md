# sema accepts _Generic with no matching type and no default — C11 requires exactly one association to match
> **Filed as [issue #1048](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1048)**

# Bug: sema accepts _Generic with no matching type and no default
**Law:** C11 6.5.1.1p2: "exactly one of the associations shall be selected" — if the selector type matches no association and there is no default, the program is ill-formed. gcc rejects with "error: _Generic selector of type int is not compatible with any association".
**Impact:** `_Generic(x, double: 1.0, char*: \str\)` with `int x` compiles silently — the result is undefined behavior. Also: ccc does not type-check non-selected branches (gcc validates ALL branches compile for the given selector type, catching bugs like `strcmp(double, double)` in the wrong branch).
**Function:** sema _Generic association matching
**Source location:** src/frontend/sema/ (no completeness check + no branch type validation)
**Detected by:** round-28 _Generic dispatch battery (20-type matrix passed except signed char #1047; this separate defect found during complex dispatch testing)
**Minimal input:** `int x = 1; _Generic(x, double: 1.0, char*: \str\);` → gcc: error; ccc: compiles (exit 0)
**Severity:** medium (silent UB — result of non-matching _Generic is undefined)
**Contract evidence:** inferred (C11 6.5.1.1p2 exactly-one-matches; gcc rejects)
**Regression test:** (deterministic: ccc -fsyntax-only must fail)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'int main(void){int x=1;_Generic(x,double:1.0,char*:\str\);return 0;}
' > /tmp/t.c
gcc -w -c -o /dev/null /tmp/t.c  # error: _Generic selector not compatible
./target/release/ccc -c -o /dev/null /tmp/t.c  # exit 0 (accepts!)
```
