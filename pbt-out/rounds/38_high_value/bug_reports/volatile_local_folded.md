# volatile LOCALS are constant-folded away at -O2 — `volatile int x=0; x=x+1; x=x+2;` keeps only the final store (C11 5.1.2.3 side-effect violation)
> **Filed as [issue #1115](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1115)**

# Bug: volatile LOCALS are constant-folded away at -O2 — `volatile int x=0; x=x+1; x=x+2;` keeps only the final store (C11 5.1.2.3 side-effect violation)
**Law:** ∀ object declared volatile, every read and write in the abstract machine is a side effect that must be preserved (C11 5.1.2.3p2, 6.7.3p8 "accesses to volatile objects are evaluated strictly according to the rules of the abstract machine"). FALSIFIED — all but the last store of a volatile local are eliminated and the reads constant-folded.
**Impact:** Breaks the canonical signal-handler / setjmp-recovery / MMIO-proxy idiom:
```c
volatile int x = 0; x = x + 1; x = x + 2; return x;
```
```
ccc  -O2: movq $1,%rax; movq $3,%rax; movl %eax,-8(%rbp); movq $3,%rax   ← init store, 2 loads, 2 stores GONE
gcc  -O2: movl $0,-4(%rsp); movl -4(%rsp),%eax; add $1; movl → ; movl ← ; add $2; movl → ; movl ←   ← all 6 accesses kept
```
Pointer-based volatile (`*vp` through `volatile int*`) is CORRECT — all dereferences survive (verified f1-f5 battery). Only non-address-taken volatile LOCALS are folded (storage promoted to a constant).
**Function:** constant folding / storage classification of volatile locals — optimizer treats a volatile local's storage as non-observable and propagates its initializer (src/passes, round-38 f6 witness)
**Detected by:** Differential — volatile access-preservation battery (round 38, /tmp/vol2.c f6; gcc -O2 vs ccc -O2 -S instruction comparison; DB had zero "volatile" coverage)
**Minimal input:** `int f(void) { volatile int x = 0; x = x + 1; return x; }` — must emit ≥2 loads + 2 stores; ccc emits only `movl $1`-style constants.
**Expected:** all 6 volatile accesses in the -O2 asm (gcc behavior above).
**Actual:** single final store; reads folded (`movq $3, %rax` returns constant 3 — which is also the WRONG value semantics: a signal handler mutating x between statements would be invisible).
**Fix sketch:** mark volatile locals' slots as non-foldable in constant propagation / mem2reg promotion (skip promotion + kill constant-fold-through for volatile-qualified alloca), mirroring how the pointer-deref path already preserves accesses.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
cat > /tmp/v.c <<'EOF'
int f6(void) { volatile int x = 0; x = x + 1; x = x + 2; return x; }
EOF
./target/debug/ccc -O2 -S /tmp/v.c -o /tmp/v_ccc.s && gcc -O2 -S /tmp/v.c -o /tmp/v_gcc.s
diff <(grep -cE "mov.-8|mov.-4" /tmp/v_ccc.s) <(grep -cE "mov.-8|mov.-4" /tmp/v_gcc.s)  # 1 vs 6 accesses
```
