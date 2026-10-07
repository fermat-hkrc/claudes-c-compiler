# x86-64: small SSE-class struct returns ({float}/{double}/{float,float} ≤ 8B) left in RAX — SysV requires XMM0; gcc interop reads garbage (mk → 0.0)
> **Filed as [issue #1114](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1114)**

# Bug: x86-64: small SSE-class struct returns ({float}/{double}/{float,float} ≤ 8B) left in RAX — SysV requires XMM0; gcc interop reads garbage (mk → 0.0)
**Law:** ∀ struct S with sizeof ≤ 8 whose single eightbyte classifies SSE (SysV AMD64 §3.2.3): a function returning S must deliver it in xmm0. FALSIFIED — ccc leaves it in rax and never writes xmm0.
**Impact:** Any cross-TU/cross-library call where a gcc- (or system-) compiled caller reads a ccc-compiled function's small float-struct return reads xmm0 = garbage:
```
$ gcc caller.c ccc_compiled_callee.o   # {float v;} / {double v;} / {float,float}
{double} take=bad mk=0.000             # arg path fine; RETURN broken
{float}  take=bad mk=0.000
{float,float} take=bad mk=(0.000000,0.000000)
{double,double} take=dead mk=2.5,3.5   # 16B (len==2) path correct
```
16-byte two-eightbyte returns and mixed INTEGER+SSE cases ARE handled; arguments are received correctly (`movq %xmm0, -8(%rbp)`). Only the ≤8B single-SSE-eightbyte RETURN path deviates — pure-ccc programs self-consistent, interop breaks.
**Function:** epilogue struct-return shuffle — src/backend/x86/codegen/returns.rs:74 (`if self.func_ret_classes.len() == 2` handles all mixed/double-SSE cases; no `len() == 1 && Sse` branch exists)
**Detected by:** Differential — gcc-caller/ccc-callee interop property p_struct_interop_gcc_calls_ccc (round 38, 250 cases; shrunk to `struct { float m0; float m1; }`)
**Minimal input:**
```c
struct S { double v; };
struct S mk(void) { struct S s = { 2.5 }; return s; }
```
**Expected:** epilogue does `movq %rax, %xmm0` (gcc -O0 mk: `movq -8(%rbp), %xmm0`), value readable by gcc caller via xmm0.
**Actual:** epilogue emits only `movq -8(%rbp), %rax; movq %rax, %r10; ret` — xmm0 never written; gcc caller does `movq %xmm0, %rax` → 0.0.
**Fix sketch:** in returns.rs, add the single-eightbyte case: `if self.func_ret_classes.len() == 1 && self.func_ret_classes[0] == Sse { emit("movq %rax, %xmm0") }` (mirroring the existing len==2 shuffles).

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- "p_struct_interop" --nocapture
# manual interop witness:
printf 'struct S { double v; };\nstruct S mk(void){ struct S s={2.5}; return s; }\nunsigned long long take(struct S s){ return s.v==2.5?0xDEADULL:0xBADULL; }' > /tmp/c.c
printf '#include <stdio.h>\nstruct S { double v; };\nextern struct S mk(void);\nextern unsigned long long take(struct S);\nint main(){struct S a=mk();printf("take=%%llx mk=%%.3f\\n",take(a),(double)a.v);}' > /tmp/m.c
./target/debug/ccc /tmp/c.c -c -o /tmp/c.o && gcc -w /tmp/m.c /tmp/c.o -o /tmp/x && /tmp/x   # take=bad mk=0.000
```
