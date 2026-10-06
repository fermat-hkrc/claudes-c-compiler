# sema accepts named zero-width bit-field (int named_zero : 0) — C11 constraint violation
> **Filed as [issue #848](https://github.com/fermat-hkrc/claudes-c-compiler/issues/848)**

# Bug: sema accepts named zero-width bit-field
**Law:** C11 6.7.2.1: a zero-width bit-field must be UNNAMED (`int : 0;` forces alignment); a NAMED bit-field with width 0 is a constraint violation. gcc rejects with "error: zero width for bit-field".
**Impact:** `struct Y { int a : 3; int named_zero : 0; int b : 5; };` compiles silently — the struct layout may differ from what a conforming compiler would produce (the named zero-width member occupies no bits but its presence affects subsequent packing in ccc's implementation).
**Function:** sema bit-field declaration handling
**Source location:** src/frontend/sema/ (named zero-width not rejected)
**Detected by:** round-26ad extreme-bitfield probe
**Minimal input:** the struct above → gcc: error; ccc: compiles
**Expected:** Err (named bit-field with width 0)
**Actual:** accepted
**Severity:** low (rare pattern; anonymous `int : 0` correctly handled — byte-identical layout verified)
**Contract evidence:** inferred (C11 6.7.2.1; gcc rejects)
**Regression test:** (deterministic: ccc -fsyntax-only on the minimal input must fail)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'struct Y { int a : 3; int named_zero : 0; int b : 5; };\nint main(void){return 0;}\n' > /tmp/t.c
gcc -w -c -o /dev/null /tmp/t.c  # error: zero width for bit-field 'named_zero'
./target/release/ccc -c -o /dev/null /tmp/t.c  # exit 0 (accepts)
```

