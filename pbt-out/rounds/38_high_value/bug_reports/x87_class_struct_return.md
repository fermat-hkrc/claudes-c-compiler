# x86-64: {long double} struct (X87 class) returned in RAX:RDX and passed in RDI:RSI — SysV requires st(0) return / memory passing; gcc interop reads -nan
> **Filed as [issue #1116](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1116)**

# Bug: x86-64: {long double} struct (X87 class) returned in RAX:RDX and passed in RDI:RSI — SysV requires st(0) return / memory passing; gcc interop reads -nan
**Law:** ∀ struct S whose eightbytes classify X87+X87UP (SysV AMD64 §3.2.3, e.g. `{ long double v; }`): a return of S must be delivered in x87 st(0), and S as an argument must be passed in MEMORY. FALSIFIED — ccc returns it in RAX:RDX (integer pair) and receives it in RDI:RSI.
**Impact:** Both directions break against gcc-compiled code:
```
$ gcc caller.c ccc_callee.o      # struct LD { long double v; }; mk returns {2.5L}
ld take=bad v=-nan                ← gcc reads st(0) (never loaded) → garbage
```
ccc mk epilogue: `popq %rax; popq %rdx; orq %rcx,%rax; orq %rsi,%rdx; ret` — x87 stack never touched.
gcc mk: `fldt LC0; fstpt -16(%rbp); fldt -16(%rbp); ret` — value in st(0).
ccc take prologue: `movq %rdi,-16(%rbp); movq %rsi,-8(%rbp)` — reads the ARG as an integer pair instead of off the stack.
**Verified-adjacent (not affected):** long double RUNTIME arithmetic is exact vs gcc (mul/div/cmp/(long long) roundtrip); >16B MEMORY-class structs (5×double) interop take=dead ✓; {double,double} SSE-pair ✓ (#1114 covers only the ≤8B single-SSE case).
**Function:** struct classification for return/args — X87/X87UP class missing from the x86-64 call-ABI handling (src/backend/call_abi.rs classify_sysv_struct family + x86 codegen return path)
**Detected by:** Differential — round-38 high-value battery manual probes (/tmp/x87_c.c + /tmp/x87_m.c), extending p_struct_interop (which caught #1114)
**Minimal input:**
```c
struct LD { long double v; };
struct LD mk(void) { struct LD s = { 2.5L }; return s; }
```
**Expected:** `fldt`/`fstpt` pair with value in st(0) at return; args via stack (gcc -O0 witness above).
**Actual:** integer-pair return RAX:RDX / integer-pair args RDI:RSI; gcc interop prints `take=bad v=-nan`.
**Fix sketch:** in classify_sysv_struct / the eightbyte classifier, emit X87 class for long-double members (SSE for float/double only), and route returns through st(0) (`fldt` of the local at epilogue), memory for args — mirroring the SSE handling added for #1114.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'struct LD { long double v; };\nstruct LD mk(void){ struct LD s={2.5L}; return s; }\nunsigned long long take(struct LD s){ return s.v==2.5L?0xDEADULL:0xBADULL; }' > /tmp/c.c
printf '#include <stdio.h>\nstruct LD { long double v; };\nextern struct LD mk(void);\nextern unsigned long long take(struct LD);\nint main(){struct LD x=mk();printf("take=%%llx v=%%.1Lf\\n",take(x),x.v);}' > /tmp/m.c
./target/debug/ccc /tmp/c.c -c -o /tmp/c.o && gcc -w /tmp/m.c /tmp/c.o -o /tmp/x && /tmp/x   # take=bad v=-nan
```
