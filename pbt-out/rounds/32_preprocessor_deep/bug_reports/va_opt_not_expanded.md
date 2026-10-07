# __VA_OPT__ not implemented in macro expansion — emitted as literal tokens into the output stream
> **Filed as [issue #1089](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1089)**

# Bug: __VA_OPT__ passes through literally

**Formal:** For every variadic macro `#define M(fmt, ...) fmt __VA_OPT__(, __VA_ARGS__)`: `M(x)` must expand without the variadic tail; `M(x, 7)` must expand with it (C23 6.10.5.2 / gcc clang behavior). FALSIFIED: the literal tokens `__VA_OPT__ ( , ... )` appear in the output stream unexpanded.

**Impact:** Token-stream differential (ccc vs gcc):
```c
#define V2(fmt, ...) fmt __VA_OPT__(| __VA_ARGS__)
V2(x);        // gcc: x            ccc: x __VA_OPT__ ( | )
V2(x, 7);     // gcc: x | 7        ccc: x __VA_OPT__ ( | 7 )
```
The unexpanded `__VA_OPT__ ( ... )` reaches the C parser — syntax errors on headers that legitimately use __VA_OPT__ (glibc <sys/cdefs.h>, common logging macros). No "unsupported feature" diagnostic either: it fails downstream with a confusing parser error.

**Scope:** __VA_OPT__ is the C23 spelling (also a widespread gcc/clang extension used since C99 days for the comma problem). 18/20 other macro-expansion corners are byte-exact vs gcc (paste with empty operands, stringize with spaces, self/mutual recursion barriers, variadic empty args, nested stringize, comments, line continuations — all verified green in the same sweep), so this is the single missing expansion feature in an otherwise solid macro engine.

**Root cause:** macro body expansion path (src/frontend/preprocessor/macro_defs.rs) has no __VA_OPT__ handling — the token is treated as an ordinary identifier.

**Separate from:** dedup "VA_OPT|va_opt|VAOPT" = 0 hits across ~1085 issues.

**Detected by:** round-32 preprocessor deep PBT — `p_macro_corners_vs_gcc` (20-form token-stream differential, 18 green / 2 fail).

**Minimal input:**
```c
#define V2(fmt, ...) fmt __VA_OPT__(, __VA_ARGS__)
V2(x);   // expands to "x __VA_OPT__ ( , )" instead of "x"
```

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- "p_macro_expansion" --nocapture
```
