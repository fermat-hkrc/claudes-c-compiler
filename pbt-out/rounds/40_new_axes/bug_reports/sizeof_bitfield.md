# sizeof on a bit-field accepted (sizeof(s.a)) — C11 6.5.3.4p1 constraint violation; gcc/clang reject
> **Filed as [issue #1136](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1136)**

# Bug: sizeof on a bit-field accepted (sizeof(s.a)) — C11 6.5.3.4p1 constraint violation; gcc/clang reject
**Law:** ∀ bit-field member m: sizeof(m) must be diagnosed (C11 6.5.3.4p1 — "The sizeof operator shall not be applied to an expression that has ... a bit-field member"). FALSIFIED — ccc compiles sizeof(s.a) and returns the declared type's size.
**Impact:** `sizeof(s.a)` silently returns sizeof(unsigned)=4 instead of erroring; the value is meaningless (a:3 occupies 3 bits) and code branching on it silently misbehaves. gcc: "error: 'sizeof' applied to a bit-field"; clang: "error: invalid application of 'sizeof' to a bit-field".
**Function:** sizeof operand validation (src/ir/lowering expr_sizeof.rs / sema) — no bit-field exclusion.
**Detected by:** round-40 expression-layer negative-contract battery.
**Minimal input:** `struct S { unsigned a:3; }; int main(void){ struct S s; return sizeof(s.a); }`
**Expected:** diagnostic (messages above; both references reject).
**Actual:** compiles; sizeof evaluates to 4.

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'struct S { unsigned a:3; };\nint main(void){ struct S s; return sizeof(s.a); }' > /tmp/s.c
gcc -w -c /tmp/s.c -o /dev/null   # error
./target/debug/ccc /tmp/s.c -c -o /tmp/s.o && echo ACCEPTED
```
