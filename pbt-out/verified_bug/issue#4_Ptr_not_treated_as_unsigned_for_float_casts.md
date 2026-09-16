# Verified Bug: Issue #4 — Ptr is not treated as U64/U32 for float and F128 casts

**Issue:** [fermat-hkrc/claudes-c-compiler#4](https://github.com/fermat-hkrc/claudes-c-compiler/issues/4)
**Verdict:** ✅ **TRUE BUG — confirmed at three levels: unit (classification), assembly (emitted code), and runtime (qemu-i386 execution).** All three minimal-input claims of the issue reproduce exactly. One impact mechanism (float→Ptr on ILP32) is **not reachable from C** — it is shielded by a separate frontend bug that silently lowers invalid float↔pointer casts as bit reinterpretation (see "New bugs discovered").
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; independently reproduced by the user on their environment.
**Verified by:** manual reproduction — Rust unit test, assembly inspection, qemu-i386-static runtime execution.
**Date:** 2026-09-15

---

## 1. The Claim

`classify_cast_with_f128` must treat `Ptr` as `U64` (LP64) / `U32` (ILP32) *before* float/F128 classification (the file's own contract, `src/backend/cast.rs:88`). Claimed failures:

1. `classify(Ptr, F32)` → `SignedToFloat { from_ty: Ptr }` (should be unsigned)
2. `classify(Ptr, F128, native)` → `SignedToF128 { from_ty: Ptr }` (should be unsigned)
3. `classify(F32, Ptr)` on ILP32 → `FloatToUnsigned { to_u64: true }` (should be `false` for 4-byte pointers)

Claimed impact: on i686, pointer→float with the high bit set converts to a **negative** value (`fildl` without unsigned correction); float→pointer on ILP32 stores an 8-byte conversion into a 4-byte pointer slot.

## 2. Evidence

### 2.1 Unit level — all three claims reproduced exactly

Scratch test `src/backend/cast.rs` → `mod scratch::manual_ptr_float_classification` (sets `target_ptr_size=4`, restores after):

```console
$ cargo test --lib backend::cast::scratch -- --nocapture
i686  Ptr->F32  = SignedToFloat { to_f64: false, from_ty: Ptr }   (expected UnsignedToFloat { to_f64: false, from_ty: U32 })
i686  Ptr->F128 = SignedToF128 { from_ty: Ptr }                    (expected UnsignedToF128 { from_ty: U32 })
i686  F32->Ptr  = FloatToUnsigned { from_f64: false, to_u64: true } (expected FloatToUnsigned { from_f64: false, to_u64: false })
LP64  Ptr->F64  = SignedToFloat { to_f64: true, from_ty: Ptr }
LP64  F64->Ptr  = FloatToUnsigned { from_f64: true, to_u64: true }

assertion `left == right` failed: ILP32: Ptr->F32 must classify like U32->F32 (unsigned)
  left: SignedToFloat { to_f64: false, from_ty: Ptr }
 right: UnsignedToFloat { to_f64: false, from_ty: U32 }
test backend::cast::scratch::manual_ptr_float_classification ... FAILED
```

### 2.2 Assembly level — misclassification reaches machine code

`/tmp/i4_p2f.c` holds a runtime pointer (`volatile`, high bit set) converted two ways: directly (`(double)p`) and via the valid unsigned route (`(double)(unsigned long)p`). Same value, two routes:

```console
$ ./target/debug/ccc-i686 -S /tmp/i4_p2f.c -o /tmp/i4_p2f.s
$ grep -nE "fildl|fildq" /tmp/i4_p2f.s
31:    fildl (%esp)     ← (double)p            — signed 32-bit load (BUG)
37:    fildq (%esp)     ← (double)(unsigned long)p — 64-bit load (correct route)
43:    fildl (%esp)     ← (float)p             — BUG
55:    fildq (%esp)     ← (float)(unsigned long)p
63:    fildl (%esp)     ← (long double)p       — BUG (F128 native path)
72:    fildl (%esp)     ← (long double)(unsigned long)p route helper
78:    fildq (%esp)     ← (long double)(unsigned long)p — correct route
```

The direct pointer→float casts emit bare `fildl` (signed 32-bit integer load) with **no unsigned correction** — exactly the `SignedToFloat { from_ty: Ptr }` → `emit_signed_to_f64` chain (`src/backend/i686/codegen/casts.rs:39-41`, `fildl` at `:178`).

### 2.3 Runtime level — qemu-i386 execution (real 80386 semantics)

`/tmp/i4_syscall.c` converts a runtime pointer `0x80000000` (high bit set) and reports a 5-bit verdict via `int $0x80` exit (no libc/crt needed):

```console
$ ./target/debug/ccc-i686 -static /tmp/i4_syscall.c -o /tmp/i4_sys
$ qemu-i386-static /tmp/i4_sys; echo "exit=$?"
exit=0          # 31 = all correct; 0 = ALL FIVE checks failed
```

| bit | check | result |
|---|---|---|
| 0 | `(double)p == (double)(unsigned long)p` | ❌ mismatch |
| 1 | `(double)p > 0` | ❌ **negative** (`fildl` reads 0x80000000 as −2147483648.0) |
| 2 | `(float)p` matches valid route | ❌ |
| 3 | `(long double)p` matches valid route (F128 native) | ❌ |
| 4 | `(void*)2147483648.0 == (void*)0x80000000` | ❌ (but see §4 — different mechanism) |

Pointer→float conversion of an address with the high bit set produces **−2147483648.0 instead of +2147483648.0** — deterministic, environment-independent miscompilation.

## 3. Root Cause

`classify_cast_with_f128` (`src/backend/cast.rs:67`) runs F128 handling and the float arms **before** Ptr normalization, and the normalization block is gated on `!from_ty.is_float() && !to_ty.is_float()` (`cast.rs:89-90`) — so any cast involving a float on either side **skips** Ptr→U32/U64 substitution:

- **Ptr→F32/F64** (`cast.rs:114`): `is_unsigned_src = from_ty.is_unsigned()` — `IrType::is_unsigned` (`src/common/types.rs:1777-1778`) matches only U8..U128, **not Ptr** → `SignedToFloat { from_ty: Ptr }`.
- **Ptr→F128 native** (`classify_f128_cast_native`, `cast.rs:158`): same `is_unsigned()` check → `SignedToF128 { from_ty: Ptr }`.
- **F32→Ptr** (`cast.rs:105`): `to_u64 = to_ty == U64 || to_ty == Ptr` — **true regardless of `target_ptr_size()`**, so on ILP32 the 64-bit conversion arm is selected.

## 4. Impact Correction (differs from the issue's claim)

The issue's float→Ptr impact ("`emit_f32_to_i64` stores an 8-byte conversion into a 4-byte pointer slot", `casts.rs:112-118, 467-478`) is **real in the classifier but unreachable from C**: the frontend silently lowers `(void*)<double>` — a C11 6.5.4 constraint violation that gcc rejects ("cannot convert to a pointer type") — as a **bit reinterpretation** (raw IEEE bits' low word used as the pointer; no `fisttpq`/conversion emitted, verified: `grep -cE "fisttp|cvt"` on the generated asm = 0). The FloatToUnsigned{to_u64:true} emitter route therefore never fires for this construct; the observable behavior is a *different* wrong result (bitcast). See New Bug B.

Everything else in the issue (pointer→float/F128 signed misclassification, including i686 `fildl` codegen and the negative value) is confirmed end-to-end.

## 5. Suggested Fix

Normalize Ptr to the integer type **first**, before all classification arms:

```rust
let ptr_int_ty = if target_is_32bit() { IrType::U32 } else { IrType::U64 };
let from_ty = if from_ty == IrType::Ptr { ptr_int_ty } else { from_ty };
let to_ty   = if to_ty   == IrType::Ptr { ptr_int_ty } else { to_ty };
```

(remove the `!is_float()` gate so the substitution also applies to float casts). This fixes all three sites at once: Ptr→float becomes unsigned, Ptr→F128 becomes unsigned, and F32→Ptr on ILP32 becomes `to_u64: false`. The scratch test in `src/backend/cast.rs` is a ready-made regression test (it passes iff the three equalities hold).

## 6. New Bugs Discovered During Verification (drafted as separate reports)

**Bug A — `ccc-i686 -static` emits no runtime: entry point = `main`.**
→ Filed as [issue #509](https://github.com/fermat-hkrc/claudes-c-compiler/issues/509) (duplicate #511 closed); full report: [`pbt-out/bug_reports/i686_static_entry_is_main_no_runtime.md`](i686_static_entry_is_main_no_runtime.md)
`int main(){return 7;}` built with `-static`: ELF entry `0x8049000`, the R E segment contains **only the 16 bytes of `main`** (`push %ebp; …; mov $7,%eax; …; ret`) — no `_start`/crt. `main`'s `ret` pops argc (=1) as the return address → jump to address 1 → SIGSEGV. Every `-static` program crashes on **any** i686 environment (confirmed under qemu-i386; independent of the missing 32-bit libc on the verification host). Dynamic builds are unaffected.

```console
$ ./target/debug/ccc-i686 -static t7.c -o t7 && qemu-i386-static ./t7
qemu: uncaught target signal 11 (Segmentation fault)      # exit=139
$ readelf -h t7 | grep Entry        → Entry point address: 0x8049000
$ readelf -l t7 | grep "R E"        → LOAD … 0x00010 0x00010 R E   # 16 bytes of code
```

**Bug B — invalid float↔pointer casts silently accepted and compiled as bit reinterpretation.**
→ Filed as [issue #510](https://github.com/fermat-hkrc/claudes-c-compiler/issues/510); full report: [`pbt-out/bug_reports/invalid_float_ptr_cast_silent_bitcast.md`](invalid_float_ptr_cast_silent_bitcast.md)
`(double)p` / `(void*)d` are C11 constraint violations (gcc: "cannot convert to a pointer type"); ccc accepts them. Pointer→float does emit a (mis-classified, per this issue) value conversion, but float→pointer emits **no conversion at all** — the raw IEEE-754 bits' low word is used as the pointer value. Either reject with a diagnostic (correct) or define semantics; current behavior is a silent, surprising bitcast. This also shields issue #4's `FloatToUnsigned{to_u64:true}` emitter route from ever firing.

## 7. Severity Assessment

Issue claims **high**. Verified: pointer→float/F128 misclassification is real, reaches machine code, and miscompiles deterministically on i686 for pointers ≥ 0x80000000 (Linux i686 user addresses occupy 0x80000000–0xBFFFFFFF, so high-bit addresses are common, e.g. mmap'ed regions). However, the constructs that expose it (`(double)ptr` etc.) are invalid C — no standards-conforming program is affected — and the float→Ptr mechanism as described is unreachable. "high" within the (self-defined) semantics of an accepting compiler is defensible; user-facing impact on valid C is currently none.

## 8. Reproduction Artifacts

- Scratch regression test: `src/backend/cast.rs` → `mod scratch::manual_ptr_float_classification` (uncommitted; passes iff #4 fixed)
- `/tmp/i4_p2f.c` — asm probe (fildl vs fildq)
- `/tmp/i4_syscall.c` — runtime probe (`int $0x80` exit, `-static`, `qemu-i386-static`); exit 0 = bug, 31 = fixed
- `/tmp/i4_f2p.c` — float→Ptr bitcast probe; `/tmp/t7.c` — Bug A probe
- Environment note: dynamic i686 binaries cannot run on the verification host (no 32-bit libc); runtime layer uses `-static` + `int $0x80` + qemu to bypass both the host limitation and Bug A.
