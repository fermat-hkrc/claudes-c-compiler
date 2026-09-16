# Verified Bug: Issue #150 — encode_ldxp_stxp emits STXP with Ws aliasing a source, with no diagnostic

**Issue:** [fermat-hkrc/claudes-c-compiler#150](https://github.com/fermat-hkrc/claudes-c-compiler/issues/150)
**Verdict:** ✅ **REAL BUG — missing diagnostic (warning-class).** Refined after full reference analysis: GNU as 2.42 **accepts but emits 3 warnings** identifying both architectural constraints; llvm-mc rejects outright; ccc accepts with **zero diagnostics**. The issue's literal claim ("must be Err", citing "GNU as / llvm-mc refuse") is half wrong — gas does not refuse — but the defect essence (no diagnostic where every reference toolchain diagnoses) is confirmed.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

ccc emits silently:

```console
$ cargo test --lib scratch_ldxp -- --nocapture
stxp w0,w0,w0,[x0] -> Ok(Word(2283798528))  [#150]    (= 0x88200000, the specified encoding)
```

GNU as (exit 0, but **3 warnings** — user-confirmed analysis):

```console
$ aarch64-linux-gnu-as /tmp/w.s -o /dev/null          # stxp w0, w0, w0, [x0]
Warning: unpredictable: identical transfer and status registers
Warning: unpredictable: identical transfer and status registers
Warning: unpredictable: identical base and status registers
```

llvm-mc/clang:

```console
error: unpredictable STXP instruction, status is also a source
```

## Analysis

Ws ∈ {Rt, Rt2} and Ws ≡ Rn (same register number) are CONSTRAINED UNPREDICTABLE. The
architecture permits encoding (gas's choice) or rejection (llvm-mc's choice) — but **both
diagnose**. ccc's assembler is the only one of the three that says nothing, so authors of
hand-written or compiler-generated asm lose the only signal that guards an
unpredictable exclusive store. Note gcc exit code alone cannot detect this class
(exit 0); the encoder must check the register constraints itself.

## Suggested Fix

In `encode_ldxp_stxp` (`load_store.rs:604+`), after resolving Ws/Rt/Rt2/Rn numbers:
emit at minimum a warning (gas parity) — or an Err (llvm-mc parity, as the issue
requests) — when `ws == rt || ws == rt2 || ws == rn`.

## Severity

Low/informational (was "high" in the issue; encoding itself is valid, but the missing
warning costs parity with both reference toolchains). Classified F7 / CWE-754.

## Artifacts

- `load_store.rs` → `mod scratch_ldxp` (#150 probe); tracker: #150 in-sample
