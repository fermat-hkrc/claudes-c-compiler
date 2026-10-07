# f128_bytes_to_x87_bytes truncates 49 mantissa bits — every long double literal on x86 is up to 1 ulp low (0.1L, pi misprinted)
> **Filed as [issue #1084](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1084)**

# Bug: f128_bytes_to_x87_bytes truncates instead of round-to-nearest

**Formal:** ∀ decimal long-double literal L with a non-representable binary expansion: bytes_ccc(L) == bytes_gcc(L) (both must be the correctly rounded x87-80 encoding of L). FALSIFIED: 7/8 sample literals are exactly 1 ulp LOW.

**Impact:** Every `long double` literal in a program compiled for x86/x86-64 is up to 1 ulp LOW. Observable at printf level:
```
printf("%.25Lf", 0.1L);
  gcc: 0.1000000000000000000013553
  ccc: 0.0999999999999999999945790    <- wrong from 20th digit
printf("%.25Lf", 3.14159265358979323846L);
  gcc: 3.1415926535897932385128090
  ccc:  3.1415926535897932382959685
```
Byte-level witnesses (16-byte x87 dump, first byte = mantissa LSB):
```
0.1L:                 gcc cd cc .. vs ccc cc cc ..
123456.789012345678L: gcc e4 69 .. vs ccc e3 69 ..
2.718281828459045235L: gcc 9b 4a .. vs ccc 9a 4a ..
999999.9999999999999L: gcc ..00 24 f4 vs ccc ..ff 23 f4
```

**Root cause:** `f128_bytes_to_x87_bytes` (src/common/long_double.rs:886) narrows the 113-bit f128 mantissa with `let mantissa64 = (mantissa >> 49) as u64;` — the comment says "Take top 64 bits". The dropped 49 bits need round-to-nearest-even (guard = bit 48, sticky = bits 47..0, plus mantissa-overflow renormalization). The decimal→f128 stage (round_to_nearest_even_113) is correct; only the second f128→x87 stage truncates.

**Blast radius:** consumers in src/backend/x86/codegen/calls.rs and src/backend/common.rs — x86-64 long-double data emission (`.tfloat`-equivalent) for every long double literal.

**Separate from:** #517 (f128_bytes_to_f64, line 906 — f64 narrowing, const-fold path), #516/#576 (subnormal paths in ir/constants.rs f64_to_* functions). Different function, different consumers, independent fix (add RNE to the 49-bit drop here).

**Fix sketch:** after `let mut m = mantissa >> 49;` apply `if (mantissa >> 48) & 1 == 1 && ((mantissa & ((1u128<<48)-1)) != 0 || m & 1 == 1) { m += 1; if m >> 64 != 0 { m = 1u64 << 63; biased_exp += 1; } }`.

**Detected by:** round-31 global-initializer PBT (byte-exact differential vs gcc, property p_global_init_bytes_vs_gcc, first shrunk case) — ir/lowering deep PBT campaign.

**Minimal input:**
```c
static long double a = 0.1L;
```

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- "pbt_global_init::p_global_init_bytes_vs_gcc" --nocapture
# E2E witness:
printf '#include <stdio.h>\nint main(){printf("%.25Lf\\n",0.1L);}' > /tmp/t.c
gcc -w -O0 -o /tmp/g /tmp/t.c && ./target/debug/ccc /tmp/t.c -o /tmp/c && /tmp/g && /tmp/c
```
