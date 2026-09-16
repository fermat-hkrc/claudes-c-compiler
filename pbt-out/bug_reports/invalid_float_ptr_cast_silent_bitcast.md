# Bug: invalid float↔pointer casts silently accepted; float→pointer compiled as raw bit reinterpretation

**Law:** C11 6.5.4 (cast operators): a cast between a pointer type and a floating type is a constraint violation — a conforming implementation must emit a diagnostic. Reference: gcc rejects both directions ("error: cannot convert to a pointer type", "pointer value used where a floating-point value was expected"); clang likewise.

**Impact:** ccc silently accepts `(double)ptr` and `(void*)dbl` (both invalid C) and compiles them with invented, asymmetric semantics:

- pointer→float emits a value conversion — but through the **mis-classified signed arm** of issue #4 (`SignedToFloat { from_ty: Ptr }` → bare `fildl` on i686), producing wrong values for pointers ≥ 0x80000000;
- float→pointer emits **no conversion at all**: the raw IEEE-754 bit pattern's low word is used as the pointer value (bit reinterpretation). `(void*)2147483648.0` therefore yields pointer `0x00000000` (low 32 bits of `0x41E0000000000000`), not any value-derived address.

This bug also **shields** the second impact mechanism claimed in issue #4 (float→Ptr on ILP32 selecting `FloatToUnsigned { to_u64: true }` → 8-byte conversion into a 4-byte slot): that classifier defect is real but this frontend behavior intercepts the construct first, so the emitter route never fires from C source.

**Function:** frontend — cast expression lowering for pointer↔floating type combinations (sema/lowering of `Cast` expressions; exact site TBD during fix)

**Detected by:** manual verification of issue #4 (fermat-hkrc/claudes-c-compiler#4).

**Minimal input:**
```c
volatile double d = 2147483648.0;
void *q = (void*)d;          /* invalid C: gcc errors; ccc accepts, q = 0x00000000 (bitcast) */
```
```c
volatile unsigned long u = 0x80000000UL;
void *p = (void*)u;
double x = (double)p;        /* invalid C: gcc errors; ccc converts as SIGNED → -2147483648.0 */
```

**Expected:** a diagnostic for both constructs (constraint violation), like gcc/clang. If ccc deliberately supports these as extensions, the semantics must be defined and documented — and pointer→float must at least be unsigned-correct (issue #4).

**Actual (i686 backend, verified):**

```console
$ gcc probe.c -o /dev/null
error: cannot convert to a pointer type                    # gcc: rejected

$ ccc-i686 -S probe.c -o probe.s && grep -cE "fisttp|cvt" probe.s
0                                                          # no conversion emitted (float->ptr = bitcast)

$ qemu-i386-static probe                                    # runtime check (see issue #4 record)
f2p mismatch: (void*)2147483648.0 != (void*)0x80000000      # q = 0x00000000
```

**Severity:** medium (silent acceptance of invalid code with surprising semantics; no conforming program is affected, but it masks issue #4's second mechanism and produces garbage pointers/NaNs with no diagnostic)

**Regression test:** `/tmp/i4_f2p.c` and `/tmp/i4_syscall.c` from the issue #4 verification record (`pbt-out/verified_bug/issue#4_Ptr_not_treated_as_unsigned_for_float_casts.md`); a minimal regression test is `assert diag((void*)1.5)` once the diagnostic exists.

**Related:** issue #4 (this bug both exposes its pointer→float half and shields its float→Ptr half).

## Found by

**Contract-Based Differential Validation (LLM verification session, 2026-09-15)** —
discovered as a side-finding while building a reproduction harness for issue #4.
(This bug itself was NOT found by pi-pbt; the command below is the campaign that found
issue #4, shown as origin context only:)

```bash
/usr/local/bin/pi-pbt build-run --provider zai-coding-cn --model glm-5.3 --lang en --build-cmd "cargo check --lib" \
  --scope src/backend/cast.rs \
  --func classify_cast_with_f128
```
