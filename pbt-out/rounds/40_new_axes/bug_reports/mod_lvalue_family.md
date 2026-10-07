# assignment/++/-- targets are never validated as modifiable lvalues — 7 constraint-violating forms compile clean (array=, cast=, func=, const elem, enum const, rvalue++, literal--)
> **Filed as [issue #1135](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1135)**

# Bug: assignment/++/-- targets are never validated as modifiable lvalues — 7 constraint-violating forms compile clean (array=, cast=, func=, const elem, enum const, rvalue++, literal--)
**Law:** ∀ assignment/incdec target T: T must be a modifiable lvalue (C11 6.5.16p1 for =, 6.5.2.4p1 for ++/--: "shall have qualified or unqualified arithmetic or pointer type and be a modifiable lvalue"). FALSIFIED — all 7 violating forms below compile with zero diagnostics.
**Impact:** 7 constraint violations accepted (gcc AND clang each reject every form with 2 errors):
```
form                     source                        gcc/clang
array assignment         int a[3],b[3]; a = b;         "array type is not assignable" / error
cast result assignment   (int)x = 5;                   "lvalue required as left operand"
function assign          f = g;                        "lvalue required"
const array element      const int a[2]={..}; a[0]=3;  "assignment of read-only location"
enum constant assign     A = 5;                        "lvalue required"
++ on rvalue             (x + 1)++;                    "lvalue required as increment operand"
-- on literal            5--;                          "lvalue required as decrement operand"
```
ccc compiles all 7 silently. The emitted behavior for e.g. `(int)x = 5` silently discards the store (or worse, writes through), `a = b` mislowers an array assignment — silent wrong-code, not just missing diagnostics.
**Function:** assignability/modifiable-lvalue check absent in sema assignment & incdec handling (src/frontend/sema/type_checker or lowering expr_assign.rs) — no lvalue-kind classification (array/function/const/enum-const/rvalue) before accepting a store.
**Detected by:** round-40 expression-layer negative-contract battery (12-form probe, gcc+clang vs ccc).
**Minimal input:** `int main(void){ int a[3], b[3]; a = b; return 0; }`
**Expected:** diagnostic per form (gcc messages above).
**Actual:** clean compile + object for all 7.

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
for t in 'int a[3],b[3]; a=b;' '(int)x=5;' 'f=g;' 'const int a[2]={1,2}; a[0]=3;' 'A=5;' '(x+1)++;' '5--;'; do
  echo "$t" | gcc -xc -w -c - -o /dev/null 2>&1 | grep -c error   # >0 every form
done
```
