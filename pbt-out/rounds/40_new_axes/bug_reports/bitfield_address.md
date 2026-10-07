# address-of on a bit-field accepted (char*)&x.b compiles — gcc/clang reject (C11 6.5.3.2p1 constraint violation)
> **Filed as [issue #1134](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1134)**

# Bug: address-of on a bit-field accepted (char*)&x.b compiles — gcc/clang reject (C11 6.5.3.2p1 constraint violation)
**Law:** ∀ bit-field member m: the unary & operator applied to m must be diagnosed (C11 6.5.3.2p1 — "the operand of the unary & operator shall be ... an lvalue that designates an object that is not a bit-field"). FALSIFIED — ccc accepts and emits an address.
**Impact:** A bit-field has no addressable byte boundary; the accepted code silently computes a pointer to the STORAGE UNIT (or an arbitrary byte), so writes through it corrupt adjacent fields:
```c
struct BF { unsigned a:3; unsigned b:5; };
char *p = (char*)&x.b;   // gcc: error: cannot take address of bit-field 'b'
                          // clang: error: address of bit-field requested
                          // ccc: compiles; p points at x's storage unit (offset observed 0)
```
**Function:** unary-& type-check in the expression lowering (src/ir/lowering expr_access / sema) — no bit-field lvalue exclusion.
**Detected by:** round-40 negative-contract probe battery (expression-layer constraints).
**Minimal input:** `struct BF { unsigned a:3; unsigned b:5; }; int main(void){ struct BF x; char *p = (char*)&x.b; return *p; }`
**Expected:** diagnostic "cannot take address of bit-field" (both references reject).
**Actual:** clean compile, object produced; observed pointer = storage-unit address (x.b shares bytes with x.a — writes through p corrupt a).

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'struct BF { unsigned a:3; unsigned b:5; };\nint main(void){ struct BF x = {5,7}; char *p = (char*)&x.b; return *p; }' > /tmp/b.c
gcc -w -c /tmp/b.c -o /dev/null   # error
clang -w -c /tmp/b.c -o /dev/null # error
./target/debug/ccc /tmp/b.c -c -o /tmp/b.o && echo ACCEPTED
```
