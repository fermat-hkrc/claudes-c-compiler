# Verified Bug: Issue #3 — float-to-unsigned casts store U8/U16 as signed I8/I16, so `to_i64()` sign-extends

**Issue:** [fermat-hkrc/claudes-c-compiler#3](https://github.com/fermat-hkrc/claudes-c-compiler/issues/3)
**Verdict:** ⚠️ **TRUE BUG at the IR representation / contract level — reproduced exactly as claimed (all four unit-level assertions of the issue confirmed).** However, no user-visible miscompilation was found: 30+ end-to-end C probes (promotion, comparisons, globals, enum members, array sizes, long long coercions, bitfields, switch, indexing, div/mod/shift, float round-trips, mid-optimization constant promotion, `-O2`) all match gcc on both debug and release builds. The defect is a **latent trap + cross-pass inconsistency**, not a demonstrated miscompile.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual reproduction — Rust unit level + E2E differential vs gcc 13.3
**Date:** 2026-09-15

---

## 1. The Claim

`IrConst::cast_float_to_target(fv, IrType::U8/U16)` stores the result in **signed narrow
variants** (`I8`/`I16`), so `to_i64()` **sign-extends** — violating:

1. `IrConst::from_i64`'s documented convention (`src/ir/constants.rs:450-461`): unsigned
   sub-64-bit constants are stored as **zero-extended `I64`**, with an explicit comment:
   *"Using I8 would sign-extend when to_i64() is called (e.g., I8(0xFF as i8) = I8(-1)
   becomes -1 instead of 255)."*
2. `cast_float_to_target`'s own docstring (`src/ir/constants.rs:273-275`): *"200.0 as u8 = 200"*.
3. The parallel fold path: `constant_fold.rs::try_fold_float_cast_mapped` (`src/passes/constant_fold.rs:601`)
   folds the **same construct** via `from_i64` → `I64(128)`.

## 2. Unit-Level Evidence (claim reproduced exactly — failing assertion)

```rust
// scratch test, src/ir/constants.rs (assert-based; FAILS on current HEAD)
let c = IrConst::cast_float_to_target(128.0, IrType::U8).unwrap();
assert_eq!(c.to_i64(), IrConst::from_i64(128, IrType::U8).to_i64(), "U8: same construct, two paths must agree");
assert_eq!(c.to_i64(), Some(128), "U8: docstring says 200.0 as u8 = 200 (no sign flip)");
```

```console
$ cargo test --lib scratch::manual_u8_u16_sign -- --nocapture

assertion `left == right` failed: U8: same construct, two paths must agree
  left: Some(-128)
 right: Some(128)
test ir::constants::scratch::manual_u8_u16_sign ... FAILED
```

Print-only observation of the same calls:

```console
$ cargo test --lib scratch::manual_u8_u16_sign -- --nocapture
cast_float_to_target(128.0, U8)  = I8(-128), to_i64 = Some(-128)
cast_float_to_target(40000.0, U16) = I16(-25536), to_i64 = Some(-25536)
cast_float_to_target(200.0, U8)  = I8(-56), to_i64 = Some(-56)
from_i64(128, U8)            = I64(128), to_i64 = Some(128)
from_i64(40000, U16)          = I64(40000), to_i64 = Some(40000)
cast_float_to_target(4294967295.0, U32) = I64(4294967295), to_i64 = Some(4294967295)
```

| Claim in issue | Reproduced | Detail |
|---|---|---|
| `cast_float_to_target(128.0, U8).to_i64() == Some(-128)` | ✅ | `I8(-128)` |
| U16 same class | ✅ | `I16(-25536)` for 40000.0 |
| `from_i64(128, U8) == I64(128)` (paths disagree) | ✅ | same construct, two representations |
| U32 already consistent | ✅ | `I64(4294967295)` |
| Docstring `200.0 as u8 = 200` violated via `to_i64()` | ✅ | `Some(-56)` |

## 3. Root Cause

`src/ir/constants.rs:283-284`:

```rust
IrType::U8  => IrConst::I8(fv as u8 as i8),   // wrapping is correct; variant is wrong
IrType::U16 => IrConst::I16(fv as u16 as i16),
```

The `as u8 as i8` chain gets **wrapping semantics right** (200.0 → 200, not saturation),
but stores the result in the **signed** variant, while `from_i64` (`src/ir/constants.rs:458,461`)
deliberately stores `I64(val as u8 as i64)` / `I64(val as u16 as i64)` for exactly this reason.
`to_i64()` (`src/ir/constants.rs:324-325`) sign-extends `I8`/`I16` variants.

**Divergent consumers of the same C construct:**

| Path | File:line | Result for `(unsigned char)128.0` |
|---|---|---|
| `constant_fold` | `src/passes/constant_fold.rs:601` (`from_i64`) | `I64(128)` ✅ |
| `simplify` | `src/passes/simplify.rs:407` (`cast_float_to_target`) | `I8(-128)` ❌ |
| frontend `const_eval` | `src/ir/lowering/const_eval.rs:268` (`cast_float_to_target`) | `I8(-128)` ❌ |

## 4. End-to-End Impact — NOT Reproduced (all probes correct)

Sharp C probes where raw `to_i64()` consumption would be visible
(`/tmp/u8fold.c`, `/tmp/u8fold2.c`):

| Probe | gcc | ccc debug | ccc release |
|---|---|---|---|
| `unsigned char c = (unsigned char)200.0; printf("%d", c)` | 200 | 200 ✅ | 200 ✅ |
| `int x = (unsigned char)200.0` | 200 | 200 ✅ | 200 ✅ |
| `int gi = (unsigned char)200.0` (global, .data) | 200 | 200 ✅ | 200 ✅ |
| `(unsigned char)200.0 < 0` (unsigned must be ≥ 0) | 0 | 0 ✅ | 0 ✅ |
| `(unsigned char)200.0 > 100` | 1 | 1 ✅ | 1 ✅ |
| `(unsigned short)40000.0 < 0` | 0 | 0 ✅ | 0 ✅ |
| `a[(unsigned char)200.0]` (index via to_i64) | 200 | 200 ✅ | 200 ✅ |
| `(unsigned char)200.0 / 3`, `% 7`, `<< 8` | 66/4/51200 | same ✅ | same ✅ |
| `(int)(double)(unsigned char)200.0` (int→float via to_i64) | 200 | 200 ✅ | 200 ✅ |
| `(unsigned char)200.0 == 200` | 1 | 1 ✅ | 1 ✅ |
| `-(int)(unsigned short)40000.0` | −40000 | −40000 ✅ | −40000 ✅ |

**Conclusion:** every probed consumer re-normalizes the constant by the *semantic type*
(sign/zero-extend per the C type) rather than trusting the `IrConst` variant, masking the
wrong representation.

**Round 2–4 probes — also all correct (gcc-matching):** enum members
(`enum { E = (unsigned char)200.0 }` → 200), folded array sizes (`int a[(unsigned char)200.0]`
→ sizeof 200), `long long`/`unsigned long` coercions (local + global), 8-bit bitfield stores,
switch subjects, **mid-optimization constant promotion** (`double d = 200.0; … (unsigned char)d`
where the cast only becomes constant after mem2reg — the realistic path for `simplify`'s fold),
and `-O2`. `/tmp/u8fold3.c`, `/tmp/u8fold4.c`.

### 4.1 Why it is masked — pass ordering

`src/passes/mod.rs:234-237`: the full-module pipeline runs `constant_fold` **before**
`simplify`. `constant_fold` folds `(unsigned char)128.0` via `from_i64` → `I64(128)` (correct),
so `simplify`'s buggy `cast_float_to_target` fold only fires on casts `constant_fold`
*declines* to fold — which for float→int is only NaN/Inf/out-of-i64-range
(`src/passes/constant_fold.rs:596-599`), i.e., C UB territory. Inside the iteration loop
(`mod.rs:385-397`) `simplify` does run before the next `constfold` round, but probes that
specifically create constants mid-optimization (`/tmp/u8fold4.c`) still produce correct
results — consumers re-normalize by semantic type regardless of which pass folded.

## 5. Residual Impact (why it is still a real bug)

1. **GVN / value-numbering divergence:** `to_hash_key()` produces `ConstHashKey::I8(-128)`
   vs `ConstHashKey::I64(128)` for the same value folded by different passes → missed CSE
   (performance), and any future logic keyed on constant equality across paths misbehaves.
2. **Latent trap:** any *new* consumer that calls `to_i64()`/`to_i128()` on these constants
   without re-truncating gets the wrong sign — exactly the class of bug the `from_i64`
   comment warns about. This is how the codebase documents the invariant; `cast_float_to_target`
   violates it.
3. **Internal inconsistency** between `simplify`/`const_eval` and `constant_fold` for the
   identical construct (also flagged by the existing TODO at `src/passes/constant_fold.rs:583`).

## 6. Suggested Fix

Make the U8/U16 arms mirror `from_i64` (`src/ir/constants.rs:283-284`):

```rust
IrType::U8  => IrConst::I64(fv as u8 as i64),
IrType::U16 => IrConst::I64(fv as u16 as i64),
```

(Wrapping semantics are already correct via `as u8`/`as u16`; only the storage variant
changes. U32/U64 arms already follow this pattern.)

## 7. Severity Assessment

Issue claims **medium**. Verified facts support "medium" as an internal-correctness /
latent-hazard rating (documented invariant violated, cross-pass representation divergence),
but **not** an active user-facing miscompile — worth noting in the issue thread that no
E2E repro exists on current HEAD, so the fix is consistency/hardening rather than
closing an observable defect.

## 8. Reproduction Artifacts

- Scratch unit test: `src/ir/constants.rs` → `mod scratch::manual_u8_u16_sign` (uncommitted)
- C probes: `/tmp/u8fold.c`, `/tmp/u8fold2.c` (contents embedded in §4 table and below)

```c
/* /tmp/u8fold2.c — sharpest probes */
int a[256]; for (int i = 0; i < 256; i++) a[i] = i;
printf("idx   = %d\n", a[(unsigned char)200.0]);              /* 200 */
printf("dbl   = %d\n", (int)(double)(unsigned char)200.0);    /* 200 */
printf("div   = %d\n", (unsigned char)200.0 / 3);             /* 66  */
printf("u16dbl= %d\n", (int)(double)(unsigned short)40000.0); /* 40000 */
```
