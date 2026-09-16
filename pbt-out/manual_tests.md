# Manual Test Commands — all 509 issues (one file)

For each issue: the probe input, the three commands, and expected outputs.
Run from the repo root. `probe.s` = the Input line written to a file.

```bash
# helper: put the input line in probe.s first, e.g.
#   echo 'INPUT LINE' > probe.s
```

## #2 — cast_float_to_target panics on f64 subnormals when targeting F128 (C-level, x86-64)

**Input (C):** `/tmp/sub2.c` — `(long double)4.9406564584124654e-324` compared against `0x1p-1074L`

```bash
# unit witness (expected: PANIC at long_double.rs:1040 — attempt to subtract with overflow)
cargo test --lib scratch::manual_f128_subnormal -- --nocapture

# E2E debug (expected: ccc crashes — 'ccc: internal error: attempt to subtract with overflow')
./target/debug/ccc /tmp/sub2.c -o /tmp/sub2_ccc && /tmp/sub2_ccc

# E2E release (expected: prints exact: 0, g = 1.112537e-308 — silent miscompile)
./target/release/ccc /tmp/sub2.c -o /tmp/sub2_rel && /tmp/sub2_rel

# gcc reference (expected: exact: 1, g = 4.940656e-324)
gcc /tmp/sub2.c -o /tmp/sub2_gcc && /tmp/sub2_gcc
```
**Verdict:** real — panic (debug) + silent miscompile (release), gcc-verified
*(probe source preserved in pbt-out/verified_bug/issue#2_...md §3.3)*

## #3 — float→unsigned casts store U8/U16 as signed I8/I16 (C-level, x86-64)

**Input (Rust unit):** `cast_float_to_target(128.0, U8)` vs `from_i64(128, U8)`

```bash
# unit witness (expected: FAIL — left Some(-128), right Some(128))
cargo test --lib scratch::manual_u8_u16_sign -- --nocapture

# E2E probes (expected: ccc matches gcc — defect latent at C level)
gcc /tmp/u8fold.c -o /tmp/g && /tmp/g && ./target/release/ccc /tmp/u8fold.c -o /tmp/c && /tmp/c
```
**Verdict:** real — IR-contract violation (latent; docstring + from_i64 convention violated)

## #4 — Ptr not treated as U64/U32 for float/F128 casts (C-level, i686)

**Input:** runtime pointer 0x80000000, direct `(double)p` vs `(double)(unsigned long)p`

```bash
# unit witness (expected: FAIL — Ptr→F32 classified SignedToFloat, etc.)
cargo test --lib backend::cast::scratch -- --nocapture

# runtime probe under qemu (expected: exit=0 — ALL 5 checks fail; 31 = correct)
./target/debug/ccc-i686 -static /tmp/i4_syscall.c -o /tmp/i4_sys && qemu-i386-static /tmp/i4_sys; echo $?

# asm evidence: bare 'fildl' (signed) for direct casts vs 'fildq' (unsigned-correct) for valid route
./target/debug/ccc-i686 -S /tmp/i4_p2f.c -o - | grep -E 'fildl|fildq'

# gcc reference (expected: exit=23 — valid routes correct; direct casts rejected at compile time)
i686-linux-gnu-gcc -O0 /tmp/issue4_valid.c -o /tmp/i4v_gnu && qemu-i386-static /tmp/i4v_gnu; echo $?
i686-linux-gnu-gcc /tmp/issue4f.c -o /dev/null 2>&1 | head -2   # 'error: pointer value used where a floating-point was expected'
```
**Verdict:** real — unit + asm + qemu-runtime; gcc encodes 0x0b2047e0 where ccc emits 0x0b0007e0

## #5 — encode_adc silently ignores a trailing shift operand

**Input:** `adc w0, w0, w0, lsl #0`

```bash
echo 'adc w0, w0, w0, lsl #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a000000
cargo test --lib scratch::manual_adc_extra_shift -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #6 — encode_adc accepts FP/SIMD register names as GPRs

**Input:** `adc d0, x1, x2`

```bash
echo 'adc d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a020020
```
**Verdict:** silent-accept

## #7 — encode_adc accepts mixed 32/64-bit register operands

**Input:** `adc w0, w0, x0`

```bash
echo 'adc w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a000000
```
**Verdict:** silent-accept

## #8 — encode_adc treats SP/WSP as XZR/WZR

**Input:** `adc wsp, w0, w0`

```bash
echo 'adc wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a00001f
```
**Verdict:** silent-accept

## #9 — ADDS/SUBS with Rd=SP/WSP encodes as XZR/WZR (CMP/CMN)

**Input:** `adds wsp, w0, #0`

```bash
echo 'adds wsp, w0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3100001f
```
**Verdict:** silent-accept

## #10 — ADD/SUB encodes FP/SIMD register names as GPRs

**Input:** `add d0, x1, x2`

```bash
echo 'add d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a scalar SIMD or floating-point register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ee28420
```
**Verdict:** silent-accept

## #11 — explicit lsl #12 ADD/SUB immediate masks overflow instead of reje

**Input:** `add w0, w1, #4097, lsl #12`

```bash
echo 'add w0, w1, #4097, lsl #12' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register, symbol or integer in range [0, 4095]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x11400420
```
**Verdict:** silent-accept

## #12 — ADD/SUB immediate form silently ignores a non-LSL#12 fourth opera

**Input:** `add w0, w0, #0, lsr #0`

```bash
echo 'add w0, w0, #0, lsr #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: only 'LSL' shift is permitted at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: only 'lsl #+N' valid after immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x11000000
```
**Verdict:** silent-accept

## #13 — ADD/SUB accepts mixed x/w register widths

**Input:** `add x0, w0, w0`

```bash
echo 'add x0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x8b000000
```
**Verdict:** silent-accept

## #14 — ADD/SUB silently encodes ROR (and other invalid shifts/extends)

**Input:** `add w0, w1, w2, ror #0`

```bash
echo 'add w0, w1, w2, ror #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: 'ROR' operator not allowed at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0b020020
cargo test --lib scratch::manual_add_sub_invalid_shift -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #15 — ADD/SUB with SP/WSP and LSL #N uses shifted-register form (XZR) i

**Input:** `add w0, wsp, w0, lsl #1`

```bash
echo 'add w0, wsp, w0, lsl #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0b0007e0
cargo test --lib scratch::manual_add_sub_sp_lsl -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** differential

## #16 — encode_adr accepts FP/SIMD register names as GPR Rd

**Input:** `adr d0, #-1048576`

```bash
echo 'adr d0, #-1048576' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x10800000
```
**Verdict:** silent-accept

## #17 — encode_adr silently truncates immediates outside the 21-bit signe

**Input:** `adr x0, #-1048577`

```bash
echo 'adr x0, #-1048577' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate out of range at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected label or encodable integer pc offset
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x707fffe0
cargo test --lib scratch::manual_adr_imm_range -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #18 — encode_adr accepts :lo12:/:got: modifiers as a bare ADR reloc

**Input:** `adr x0, :lo12:foo`

```bash
echo 'adr x0, :lo12:foo' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: this relocation modifier is not allowed on this instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected adr label
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x10000000
cargo test --lib scratch::manual_adr_modifier -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #19 — encode_adr treats SP as XZR

**Input:** `adr sp, #-1048576`

```bash
echo 'adr sp, #-1048576' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1080001f
```
**Verdict:** silent-accept

## #20 — encode_adr accepts 32-bit W registers as ADR Rd

**Input:** `adr w0, #-1048576`

```bash
echo 'adr w0, #-1048576' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x10800000
```
**Verdict:** silent-accept

## #21 — encode_bic accepts FP/SIMD register names as GPRs

**Input:** `bic d0, x1, x2`

```bash
echo 'bic d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a220020
```
**Verdict:** silent-accept

## #22 — encode_bic immediate form treats XZR/WZR as SP/WSP

**Input:** `bic wzr, w0, #1`

```bash
echo 'bic wzr, w0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x121f781f
```
**Verdict:** silent-accept

## #23 — encode_bic NEON form accepts arrangements other than 8b/16b

**Input:** `bic v0.8h, v1.8h, v2.8h`

```bash
echo 'bic v0.8h, v1.8h, v2.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e621c20
```
**Verdict:** silent-accept

## #24 — encode_bic accepts mixed-width GPR operands

**Input:** `bic w0, w0, x0`

```bash
echo 'bic w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a200000
cargo test --lib scratch::manual_bic_mixed_width -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #25 — encode_bic masks out-of-range shift amounts instead of rejecting 

**Input:** `bic w0, w0, w0, lsl #32`

```bash
echo 'bic w0, w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a208000
cargo test --lib scratch::manual_bic_shift_range -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #26 — encode_bic register form accepts SP/WSP as a GPR

**Input:** `bic wsp, w0, w0`

```bash
echo 'bic wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a20001f
```
**Verdict:** silent-accept

## #27 — encode_bic maps unknown shift kinds to LSL

**Input:** `bic w0, w0, w0, lslx #0`

```bash
echo 'bic w0, w0, w0, lslx #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a200000
```
**Verdict:** silent-accept

## #28 — encode_bics ignores a trailing non-shift 4th operand

**Input:** `bics w0, w0, w0, w0`

```bash
echo 'bics w0, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a200000
```
**Verdict:** silent-accept

## #29 — encode_bics accepts FP/SIMD register names as GPRs

**Input:** `bics d0, x1, x2`

```bash
echo 'bics d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or predicate register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a220020
```
**Verdict:** silent-accept

## #30 — encode_bics rejects the GNU BICS-immediate alias

**Input:** `bics w0, w0, #0xaaaaaaaa`

```bash
echo 'bics w0, w0, #0xaaaaaaaa' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected register at operand 2, got Some(Imm(2863311530))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** not-a-defect (gas parity): llvm-mc-only alias — enhancement request

## #31 — encode_bics accepts mixed-width GPR operands

**Input:** `bics w0, w0, x0`

```bash
echo 'bics w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a200000
```
**Verdict:** silent-accept

## #32 — encode_bics accepts out-of-range shift amounts

**Input:** `bics w0, w0, w0, lsl #32`

```bash
echo 'bics w0, w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a208000
```
**Verdict:** silent-accept

## #33 — encode_bics accepts SP/WSP as register 31

**Input:** `bics wsp, w0, w0`

```bash
echo 'bics wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or predicate register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a20001f
```
**Verdict:** silent-accept

## #34 — encode_bics treats unknown shift kinds as LSL

**Input:** `bics w0, w0, w0, lslx #0`

```bash
echo 'bics w0, w0, w0, lslx #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6a200000
```
**Verdict:** silent-accept

## #35 — encode_bl ignores extra operands

**Input:** `bl labl0, x0`

```bash
echo 'bl labl0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x94000000
```
**Verdict:** silent-accept

## #36 — encode_bl rejects immediate PC-offset form `bl #imm`

**Input:** `bl #-134217728`

```bash
echo 'bl #-134217728' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected symbol at operand 0, got Some(Imm(-134217728))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** reverse

## #37 — encode_bl accepts :lo12: / modifier operands as Call26 symbols

**Input:** `bl :lo12:foo`

```bash
echo 'bl :lo12:foo' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: unsupported instruction: foo
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unknown mnemonic `foo'
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic
cargo test --lib scratch::manual_bl_modifier -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** real — parser-masked at CLI (unit-proven: encoder accepts, sample #37)

## #38 — encode_neon_three_diff_narrow silently ignores a fourth operand

**Input:** `addhn v0.8b, v0.8h, v0.8h, v0.8h`

```bash
echo 'addhn v0.8b, v0.8h, v0.8h, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e204000
cargo test --lib scratch::manual_addhn_four_operands -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #39 — encode_neon_three_diff_narrow accepts GPR/FP names as NEON Vd

**Input:** `addhn x0, v0.8h, v0.8h`

```bash
echo 'addhn x0, v0.8h, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e204000
```
**Verdict:** silent-accept

## #40 — encode_neon_three_diff_narrow ignores destination arrangement Tb

**Input:** `addhn2 v0.8b, v0.8h, v0.8h`

```bash
echo 'addhn2 v0.8b, v0.8h, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e204000
```
**Verdict:** silent-accept

## #41 — encode_neon_three_diff_narrow ignores Rm arrangement Ta

**Input:** `addhn v0.4h, v0.4s, v0.8h`

```bash
echo 'addhn v0.4h, v0.4s, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e604000
```
**Verdict:** silent-accept

## #42 — encode_blr ignores extra operands

**Input:** `blr x0, x1`

```bash
echo 'blr x0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd63f0000
cargo test --lib scratch_blr -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #43 — encode_blr accepts FP/SIMD register names as GPR Rn

**Input:** `blr d0`

```bash
echo 'blr d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd63f0000
```
**Verdict:** silent-accept

## #44 — encode_blr treats SP as XZR

**Input:** `blr sp`

```bash
echo 'blr sp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd63f03e0
```
**Verdict:** silent-accept

## #45 — encode_blr accepts 32-bit W registers as BLR Rn

**Input:** `blr w0`

```bash
echo 'blr w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd63f0000
```
**Verdict:** silent-accept

## #46 — encode_br ignores extra operands

**Input:** `br x0, x1`

```bash
echo 'br x0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd61f0000
cargo test --lib scratch_br -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #47 — encode_br accepts FP/SIMD register names as GPR Rn

**Input:** `br d0`

```bash
echo 'br d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd61f0000
```
**Verdict:** silent-accept

## #48 — encode_br treats SP as XZR

**Input:** `br sp`

```bash
echo 'br sp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd61f03e0
```
**Verdict:** silent-accept

## #49 — encode_br accepts 32-bit W registers as BR Rn

**Input:** `br w0`

```bash
echo 'br w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd61f0000
cargo test --lib scratch_br -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #50 — encode_branch ignores extra operands

**Input:** `b labl0, x0`

```bash
echo 'b labl0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x14000000
```
**Verdict:** silent-accept

## #51 — encode_branch rejects immediate PC-offset form `b #imm`

**Input:** `b #0`

```bash
echo 'b #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected symbol at operand 0, got Some(Imm(0))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
cargo test --lib scratch_br -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** reverse

## #52 — encode_branch accepts :lo12: / modifier operands as Jump26 symbol

**Input:** `b :lo12:foo`

```bash
echo 'b :lo12:foo' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: unsupported instruction: foo
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unknown mnemonic `foo'
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic
```
**Verdict:** real — parser-masked at CLI (encoder accepts per issue; parser rejects :lo12: text)

## #53 — encode_cbz ignores extra operands

**Input:** `cbz x0, L, x1`

```bash
echo 'cbz x0, L, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb4000020
cargo test --lib scratch_cbz -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #54 — encode_cbz accepts FP/SIMD register names as Rt

**Input:** `cbz d0, L`

```bash
echo 'cbz d0, L' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x34000020
cargo test --lib scratch_cbz -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #55 — encode_cbz rejects immediate PC-offset form `cbz/cbnz Rt, #imm`

**Input:** `cbz x0, #-1048576`

```bash
echo 'cbz x0, #-1048576' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected symbol at operand 1, got Some(Imm(-1048576))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** reverse

## #56 — encode_cbz treats SP as XZR

**Input:** `cbz sp, L`

```bash
echo 'cbz sp, L' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb400003f
```
**Verdict:** silent-accept

## #57 — encode_ccmp_ccmn ignores extra operands

**Input:** `ccmn x0, #0, #0, eq, x1`

```bash
echo 'ccmn x0, #0, #0, eq, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xba400800
```
**Verdict:** silent-accept

## #58 — encode_ccmp_ccmn accepts FP/SIMD register names as GPRs

**Input:** `ccmp d0, #0, #0, eq`

```bash
echo 'ccmp d0, #0, #0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x7a400800
```
**Verdict:** silent-accept

## #59 — encode_ccmp_ccmn truncates out-of-range imm5 and nzcv instead of 

**Input:** `ccmn x0, #-1, #0, eq`

```bash
echo 'ccmn x0, #-1, #0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 31 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xba5f0800
cargo test --lib scratch_ccmp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #60 — encode_ccmp_ccmn accepts mixed x/w register widths

**Input:** `ccmn w0, x0, #0, eq`

```bash
echo 'ccmn w0, x0, #0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3a400000
```
**Verdict:** silent-accept

## #61 — encode_ccmp_ccmn encodes SP/WSP as XZR/WZR

**Input:** `ccmn sp, #0, #0, eq`

```bash
echo 'ccmn sp, #0, #0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xba400be0
```
**Verdict:** silent-accept

## #62 — encode_cinc accepts condition codes AL and NV

**Input:** `cinc x0, x0, al`

```bash
echo 'cinc x0, x0, al' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand 3 must be one of the standard conditions, excluding AL and NV.
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: condition codes AL and NV are invalid for this instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a80f400
```
**Verdict:** silent-accept

## #63 — encode_cinc ignores extra operands

**Input:** `cinc x0, x0, eq, x2`

```bash
echo 'cinc x0, x0, eq, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a801400
```
**Verdict:** silent-accept

## #64 — encode_cinc accepts FP/SIMD register names as GPRs

**Input:** `cinc d0, d0, eq`

```bash
echo 'cinc d0, d0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a801400
```
**Verdict:** silent-accept

## #65 — encode_cinc accepts mixed x/w register widths

**Input:** `cinc x0, w0, eq`

```bash
echo 'cinc x0, w0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a801400
cargo test --lib scratch_width -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #66 — encode_cinc encodes SP/WSP as XZR/WZR

**Input:** `cinc sp, x0, eq`

```bash
echo 'cinc sp, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a80141f
```
**Verdict:** silent-accept

## #67 — encode_cinv accepts condition codes AL and NV

**Input:** `cinv x0, x0, al`

```bash
echo 'cinv x0, x0, al' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand 3 must be one of the standard conditions, excluding AL and NV.
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: condition codes AL and NV are invalid for this instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80f000
```
**Verdict:** silent-accept

## #68 — encode_cinv ignores extra operands

**Input:** `cinv x0, x0, eq, x2`

```bash
echo 'cinv x0, x0, eq, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda801000
```
**Verdict:** silent-accept

## #69 — encode_cinv accepts FP/SIMD register names as GPRs

**Input:** `cinv d0, d0, eq`

```bash
echo 'cinv d0, d0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a801000
```
**Verdict:** silent-accept

## #70 — encode_cinv accepts mixed x/w register widths

**Input:** `cinv x0, w0, eq`

```bash
echo 'cinv x0, w0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda801000
```
**Verdict:** silent-accept

## #71 — encode_cinv encodes SP/WSP as XZR/WZR

**Input:** `cinv sp, x0, eq`

```bash
echo 'cinv sp, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80101f
```
**Verdict:** silent-accept

## #72 — encode_cmn ignores extra operands

**Input:** `cmn x0, x0, x2`

```bash
echo 'cmn x0, x0, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xab00001f
```
**Verdict:** silent-accept

## #73 — encode_cmn accepts FP/SIMD register names as GPRs

**Input:** `cmn d0, #0`

```bash
echo 'cmn d0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb100001f
cargo test --lib scratch_width -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #74 — encode_cmn panics on Imm(i64::MIN)

**Input:** `cmn x0, #i64::MIN`

```bash
echo 'cmn x0, #i64::MIN' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: failed to resolve deferred instruction 'cmn': unsupported add/sub operands: [Reg("xzr"), R
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd503201f
```
**Verdict:** real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue)

## #75 — encode_cmn accepts mixed x/w register widths

**Input:** `cmn x0, w0`

```bash
echo 'cmn x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: missing extend operator at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xab00001f
```
**Verdict:** silent-accept

## #76 — encode_cmn encodes SP as Rm as XZR

**Input:** `cmn x0, sp`

```bash
echo 'cmn x0, sp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register, symbol or integer in range [0, 4095]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xab1f001f
```
**Verdict:** silent-accept

## #77 — encode_cmn accepts XZR/WZR as immediate-form Rn

**Input:** `cmn xzr, #0`

```bash
echo 'cmn xzr, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb10003ff
```
**Verdict:** silent-accept

## #78 — encode_cmp ignores extra operands

**Input:** `cmp x0, x0, x2`

```bash
echo 'cmp x0, x0, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xeb00001f
```
**Verdict:** silent-accept

## #79 — encode_cmp accepts FP/SIMD register names as GPRs

**Input:** `cmp d0, #0`

```bash
echo 'cmp d0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf100001f
```
**Verdict:** silent-accept

## #80 — encode_cmp panics on Imm(i64::MIN)

**Input:** `cmp x0, #i64::MIN`

```bash
echo 'cmp x0, #i64::MIN' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: failed to resolve deferred instruction 'cmp': unsupported add/sub operands: [Reg("xzr"), R
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd503201f
```
**Verdict:** real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue)

## #81 — encode_cmp accepts mixed x/w without an extend

**Input:** `cmp x0, w0`

```bash
echo 'cmp x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: missing extend operator at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xeb00001f
cargo test --lib scratch_width -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #82 — encode_cmp accepts SP as Rm without an extend

**Input:** `cmp x0, sp`

```bash
echo 'cmp x0, sp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register, symbol or integer in range [0, 4095]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xeb1f001f
```
**Verdict:** silent-accept

## #83 — encode_cmp accepts XZR/WZR as immediate-form Rn

**Input:** `cmp xzr, #0`

```bash
echo 'cmp xzr, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf10003ff
cargo test --lib scratch_cmp_zr -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #84 — encode_cneg accepts condition codes AL and NV

**Input:** `cneg x0, x0, al`

```bash
echo 'cneg x0, x0, al' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand 3 must be one of the standard conditions, excluding AL and NV.
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: condition codes AL and NV are invalid for this instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80f400
```
**Verdict:** silent-accept

## #85 — encode_cneg ignores extra operands

**Input:** `cneg x0, x0, eq, x2`

```bash
echo 'cneg x0, x0, eq, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda801400
cargo test --lib scratch_cneg -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #86 — encode_cneg accepts FP/SIMD register names as GPRs

**Input:** `cneg d0, d0, eq`

```bash
echo 'cneg d0, d0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a801400
```
**Verdict:** silent-accept

## #87 — encode_cneg accepts mixed x/w register widths

**Input:** `cneg x0, w0, eq`

```bash
echo 'cneg x0, w0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda801400
```
**Verdict:** silent-accept

## #88 — encode_cneg encodes SP/WSP as XZR/WZR

**Input:** `cneg sp, x0, eq`

```bash
echo 'cneg sp, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80141f
```
**Verdict:** silent-accept

## #89 — encode_csel ignores extra operands

**Input:** `csel x0, x1, x2, eq, x3`

```bash
echo 'csel x0, x1, x2, eq, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a820020
```
**Verdict:** silent-accept

## #90 — encode_csel accepts FP/SIMD register names as GPRs

**Input:** `csel d0, d1, d2, eq`

```bash
echo 'csel d0, d1, d2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a820020
```
**Verdict:** silent-accept

## #91 — encode_csel accepts mixed x/w register widths

**Input:** `csel x0, w1, x2, eq`

```bash
echo 'csel x0, w1, x2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a820020
```
**Verdict:** silent-accept

## #92 — encode_csel encodes SP as XZR

**Input:** `csel sp, x0, x1, eq`

```bash
echo 'csel sp, x0, x1, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a81001f
```
**Verdict:** silent-accept

## #93 — encode_cset accepts condition codes AL and NV

**Input:** `cset x0, al`

```bash
echo 'cset x0, al' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand 2 must be one of the standard conditions, excluding AL and NV.
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: condition codes AL and NV are invalid for this instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a9ff7e0
```
**Verdict:** silent-accept

## #94 — encode_cset ignores extra operands

**Input:** `cset x0, eq, x2`

```bash
echo 'cset x0, eq, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a9f17e0
```
**Verdict:** silent-accept

## #95 — encode_cset accepts FP/SIMD register names as GPRs

**Input:** `cset d0, eq`

```bash
echo 'cset d0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a9f17e0
```
**Verdict:** silent-accept

## #96 — encode_cset encodes SP as XZR

**Input:** `cset sp, eq`

```bash
echo 'cset sp, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a9f17ff
```
**Verdict:** silent-accept

## #97 — encode_csetm accepts condition codes AL and NV

**Input:** `csetm x0, al`

```bash
echo 'csetm x0, al' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand 2 must be one of the standard conditions, excluding AL and NV.
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: condition codes AL and NV are invalid for this instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda9ff3e0
```
**Verdict:** silent-accept

## #98 — encode_csetm ignores extra operands

**Input:** `csetm x0, eq, x2`

```bash
echo 'csetm x0, eq, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda9f13e0
```
**Verdict:** silent-accept

## #99 — encode_csetm accepts FP/SIMD register names as GPRs

**Input:** `csetm d0, eq`

```bash
echo 'csetm d0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a9f13e0
```
**Verdict:** silent-accept

## #100 — encode_csetm encodes SP as XZR

**Input:** `csetm sp, eq`

```bash
echo 'csetm sp, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda9f13ff
cargo test --lib scratch_cond_ops -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #101 — encode_csinc ignores extra operands

**Input:** `csinc x0, x0, x0, eq, x3`

```bash
echo 'csinc x0, x0, x0, eq, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a800400
```
**Verdict:** silent-accept

## #102 — encode_csinc accepts FP/SIMD register names as GPRs

**Input:** `csinc d0, d1, d2, eq`

```bash
echo 'csinc d0, d1, d2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1a820420
```
**Verdict:** silent-accept

## #103 — encode_csinc accepts mixed x/w register widths

**Input:** `csinc x0, w1, x2, eq`

```bash
echo 'csinc x0, w1, x2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a820420
cargo test --lib scratch_cond_ops -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #104 — encode_csinc encodes SP as XZR

**Input:** `csinc sp, x0, x0, eq`

```bash
echo 'csinc sp, x0, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9a80041f
```
**Verdict:** silent-accept

## #105 — encode_csinv ignores extra operands

**Input:** `csinv x0, x0, x0, eq, x3`

```bash
echo 'csinv x0, x0, x0, eq, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda800000
```
**Verdict:** silent-accept

## #106 — encode_csinv accepts FP/SIMD register names as GPRs

**Input:** `csinv d0, d1, d2, eq`

```bash
echo 'csinv d0, d1, d2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a820020
```
**Verdict:** silent-accept

## #107 — encode_csinv accepts mixed x/w register widths

**Input:** `csinv x0, w1, x2, eq`

```bash
echo 'csinv x0, w1, x2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda820020
```
**Verdict:** silent-accept

## #108 — encode_csinv encodes SP as XZR

**Input:** `csinv sp, x0, x0, eq`

```bash
echo 'csinv sp, x0, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80001f
```
**Verdict:** silent-accept

## #109 — encode_csneg ignores extra operands

**Input:** `csneg x0, x0, x0, eq, x3`

```bash
echo 'csneg x0, x0, x0, eq, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda800400
cargo test --lib scratch_cond_ops -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #110 — encode_csneg encodes FP/SIMD register names as GPRs

**Input:** `csneg d0, d1, d2, eq`

```bash
echo 'csneg d0, d1, d2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a820420
```
**Verdict:** silent-accept

## #111 — encode_csneg accepts mixed x/w register widths

**Input:** `csneg x0, w1, x2, eq`

```bash
echo 'csneg x0, w1, x2, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda820420
```
**Verdict:** silent-accept

## #112 — encode_csneg encodes SP as XZR

**Input:** `csneg sp, x0, x0, eq`

```bash
echo 'csneg sp, x0, x0, eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xda80041f
cargo test --lib scratch_csneg_sp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #113 — encode_div silently ignores a fourth operand

**Input:** `sdiv w0, w0, w0, x0`

```bash
echo 'sdiv w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1ac00c00
cargo test --lib scratch_div -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #114 — encode_div accepts FP/SIMD register names as GPRs

**Input:** `sdiv d0, x1, x2`

```bash
echo 'sdiv d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1ac20c20
cargo test --lib scratch_div -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #115 — encode_div accepts mixed x/w register widths

**Input:** `sdiv w0, w0, x0`

```bash
echo 'sdiv w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1ac00c00
```
**Verdict:** silent-accept

## #116 — encode_div accepts SP/WSP as a GPR operand

**Input:** `sdiv wsp, w0, w0`

```bash
echo 'sdiv wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1ac00c1f
cargo test --lib scratch_div -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #117 — encode_eon ignores a trailing non-shift 4th operand

**Input:** `eon w0, w0, w0, w0`

```bash
echo 'eon w0, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a200000
```
**Verdict:** silent-accept

## #118 — encode_eon accepts FP/SIMD register names as GPRs

**Input:** `eon d0, x1, x2`

```bash
echo 'eon d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a220020
cargo test --lib scratch_eon -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #119 — encode_eon rejects the GNU EON-immediate alias

**Input:** `eon w0, w0, #0xaaaaaaaa`

```bash
echo 'eon w0, w0, #0xaaaaaaaa' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected register at operand 2, got Some(Imm(2863311530))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** not-a-defect (gas parity): llvm-mc-only alias — enhancement request

## #120 — encode_eon accepts mixed x/w register widths

**Input:** `eon w0, w0, x0`

```bash
echo 'eon w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a200000
```
**Verdict:** silent-accept

## #121 — encode_eon masks out-of-range shift amounts instead of rejecting 

**Input:** `eon w0, w0, w0, lsl #32`

```bash
echo 'eon w0, w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a208000
cargo test --lib scratch_eon -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #122 — encode_eon encodes SP/WSP as XZR/WZR

**Input:** `eon wsp, w0, w0`

```bash
echo 'eon wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a20001f
```
**Verdict:** silent-accept

## #123 — encode_eon treats unknown shift kinds as LSL

**Input:** `eon w0, w0, w0, lslx #0`

```bash
echo 'eon w0, w0, w0, lslx #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4a200000
```
**Verdict:** silent-accept

## #124 — encode_ldar_stlr ignores extra operands

**Input:** `stlr w0, [x0], x2`

```bash
echo 'stlr w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected address writeback at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x889ffc00
```
**Verdict:** silent-accept

## #125 — encode_ldar_stlr accepts SP/WSP as Rt

**Input:** `stlr sp, [x0]`

```bash
echo 'stlr sp, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc89ffc1f
```
**Verdict:** silent-accept

## #126 — encode_ldar_stlr accepts a W register as the memory base

**Input:** `stlr w0, [w0]`

```bash
echo 'stlr w0, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x889ffc00
```
**Verdict:** silent-accept

## #127 — encode_neon_across_long ignores destination register type

**Input:** `saddlv b0, v0.8b`

```bash
echo 'saddlv b0, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e303800
cargo test --lib scratch_saddlv -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #128 — encode_neon_across_long ignores extra operands

**Input:** `saddlv h0, v0.8b, h0`

```bash
echo 'saddlv h0, v0.8b, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e303800
```
**Verdict:** silent-accept

## #129 — encode_neon_across_long encodes reserved SADDLV arrangements (2S/

**Input:** `saddlv h0, v0.2s`

```bash
echo 'saddlv h0, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0eb03800
```
**Verdict:** silent-accept

## #130 — encode_neon_float_cmp_zero ignores source arrangement

**Input:** `fcmeq v0.4s, v0.2s, #0.0`

```bash
echo 'fcmeq v0.4s, v0.2s, #0.0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd503201f
```
**Verdict:** real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue)

## #131 — encode_neon_float_cmp_zero ignores extra operands

**Input:** `fcmeq v0.2s, v0.2s, #0.0, v0.2s`

```bash
echo 'fcmeq v0.2s, v0.2s, #0.0, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd503201f
```
**Verdict:** real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue)

## #132 — encode_neon_float_cmp_zero accepts non-V register prefixes

**Input:** `fcmeq x0.2s, v0.2s, #0.0`

```bash
echo 'fcmeq x0.2s, v0.2s, #0.0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd503201f
```
**Verdict:** real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue)

## #133 — encode_neon_sli ignores source arrangement

**Input:** `sli v0.8b, v0.16b, #0`

```bash
echo 'sli v0.8b, v0.16b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f085400
```
**Verdict:** silent-accept

## #134 — encode_neon_sli ignores extra operands

**Input:** `sli v0.8b, v0.8b, #0, v0.8b`

```bash
echo 'sli v0.8b, v0.8b, #0, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f085400
```
**Verdict:** silent-accept

## #135 — encode_neon_sli accepts non-V register prefixes as V registers

**Input:** `sli x0.8b, v0.8b, #0`

```bash
echo 'sli x0.8b, v0.8b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f085400
```
**Verdict:** silent-accept

## #136 — encode_neon_sli does not reject out-of-range shift

**Input:** `sli v0.8b, v0.8b, #-1`

```bash
echo 'sli v0.8b, v0.8b, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to add with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 7 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 7].
```
**Verdict:** panic-on-invalid

## #137 — encode_ldur_stur ignores extra operands

**Input:** `stur w0, [x0, #-256], x2`

```bash
echo 'stur w0, [x0, #-256], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: cannot combine pre- and post-indexing at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb8100000
cargo test --lib scratch_ldur -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #138 — encode_ldur_stur silently wraps offsets outside signed 9-bit rang

**Input:** `stur w0, [x0, #-257]`

```bash
echo 'stur w0, [x0, #-257]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate offset out of range -256 to 255 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be an integer in range [-256, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb80ff000
cargo test --lib scratch_ldur -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #139 — encode_ldur_stur encodes LR as a 32-bit W30 load/store

**Input:** `stur lr, [x0, #-256]`

```bash
echo 'stur lr, [x0, #-256]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb810001e
```
**Verdict:** differential

## #140 — encode_ldur_stur encodes SIMD Rt on LDTR/STTR

**Input:** `ldtr d0, [x0]`

```bash
echo 'ldtr d0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xfc400800
```
**Verdict:** silent-accept

## #141 — encode_ldur_stur accepts SP/WSP as Rt

**Input:** `stur sp, [x0, #-256]`

```bash
echo 'stur sp, [x0, #-256]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xbc10001f
```
**Verdict:** silent-accept

## #142 — encode_ldur_stur encodes a V-register Rt as a 64-bit SIMD (D) loa

**Input:** `ldur v0, [x0]`

```bash
echo 'ldur v0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xfc400000
cargo test --lib scratch_ldxp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #143 — encode_ldur_stur accepts a W register (or XZR/WZR/WSP) as the mem

**Input:** `ldur x0, [w0]`

```bash
echo 'ldur x0, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf8400000
```
**Verdict:** silent-accept

## #144 — encode_ldxp_stxp ignores extra operands

**Input:** `stxp w0, w0, w0, [x0], x2`

```bash
echo 'stxp w0, w0, w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88200000
cargo test --lib scratch_ldxp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #145 — encode_ldxp_stxp encodes SIMD/FP registers as GPRs

**Input:** `ldxp d0, x1, [x2]`

```bash
echo 'ldxp d0, x1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x887f0440
```
**Verdict:** silent-accept

## #146 — encode_ldxp_stxp accepts a mixed X/W exclusive pair

**Input:** `ldxp x0, w1, [x2]`

```bash
echo 'ldxp x0, w1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc87f0440
```
**Verdict:** silent-accept

## #147 — encode_ldxp_stxp ignores a nonzero exclusive-pair offset

**Input:** `stxp w0, w0, w0, [x0, #-1]`

```bash
echo 'stxp w0, w0, w0, [x0, #-1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the optional immediate offset can only be 0 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be absent or #0
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88200000
```
**Verdict:** silent-accept

## #148 — encode_ldxp_stxp encodes SP/WSP as ZR in Rt/Rt2/Ws

**Input:** `stxp w0, sp, w0, [x0]`

```bash
echo 'stxp w0, sp, w0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc820001f
```
**Verdict:** silent-accept

## #149 — encode_ldxp_stxp accepts a W register as the memory base

**Input:** `ldxp w0, w1, [w0]`

```bash
echo 'ldxp w0, w1, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x887f0400
```
**Verdict:** silent-accept

## #150 — encode_ldxp_stxp encodes STXP when Ws aliases a source

**Input:** `stxp w0, w0, w0, [x0]`

```bash
echo 'stxp w0, w0, w0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: unpredictable: identical transfer and status registers
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unpredictable STXP instruction, status is also a source
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88200000
cargo test --lib scratch_ldxp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** warning-class-CU

## #151 — encode_ldxp_stxp accepts an X register as STXP status

**Input:** `stxp x0, x1, x2, [x3]`

```bash
echo 'stxp x0, x1, x2, [x3]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8200861
```
**Verdict:** silent-accept

## #152 — encode_ldxp_stxp encodes XZR/WZR as SP when used as the base

**Input:** `ldxp x0, x1, [xzr]`

```bash
echo 'ldxp x0, x1, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc87f07e0
cargo test --lib scratch_ldxp2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #153 — encode_neon_float_three_same ignores source arrangement and accep

**Input:** `fadd v0.2d, v0.2s, v0.2s`

```bash
echo 'fadd v0.2d, v0.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e60d400
```
**Verdict:** silent-accept

## #154 — encode_neon_float_three_same ignores extra operands

**Input:** `fadd v0.2s, v0.2s, v0.2s, v0.2s`

```bash
echo 'fadd v0.2s, v0.2s, v0.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e20d400
```
**Verdict:** silent-accept

## #155 — encode_neon_float_three_same encodes non-V register prefixes as V

**Input:** `fadd x0.2s, v0.2s, v0.2s`

```bash
echo 'fadd x0.2s, v0.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e20d400
```
**Verdict:** silent-accept

## #156 — encode_neon_float_three_same accepts a source without arrangement

**Input:** `fadd v0.2s, v0, v0.2s`

```bash
echo 'fadd v0.2s, v0, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e20d400
```
**Verdict:** silent-accept

## #157 — x86 encoder does not validate register size against mnemonic suff

**Input:** `movq %sp, %rax`

```bash
echo 'movq %sp, %rax' > probe.s
./target/debug/ccc -c probe.s -o probe_c.o     # ccc expected:  accepts
gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand type mismatch for `movq'
clang -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x48 89 e0
```
**Verdict:** silent-accept

## #158 — encode_ldxr_stxr ignores extra operands

**Input:** `stxr w0, w0, [x0], x2`

```bash
echo 'stxr w0, w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88007c00
```
**Verdict:** silent-accept

## #159 — encode_ldxr_stxr encodes SIMD/FP names as GPR Rt

**Input:** `ldxr d0, [x1]`

```bash
echo 'ldxr d0, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x885f7c20
```
**Verdict:** silent-accept

## #160 — encode_ldxr_stxr ignores a nonzero exclusive offset

**Input:** `stxr w1, x0, [x2, #-1]`

```bash
echo 'stxr w1, x0, [x2, #-1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the optional immediate offset can only be 0 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be absent or #0
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8017c40
```
**Verdict:** silent-accept

## #161 — encode_ldxr_stxr encodes SP as ZR in Rt

**Input:** `ldxr sp, [x0]`

```bash
echo 'ldxr sp, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85f7c1f
```
**Verdict:** silent-accept

## #162 — encode_ldxr_stxr accepts a W register as the address base

**Input:** `ldxr x0, [w1]`

```bash
echo 'ldxr x0, [w1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85f7c20
```
**Verdict:** silent-accept

## #163 — encode_ldxr_stxr encodes STXR when Ws aliases a source

**Input:** `stxr w0, w0, [x0]`

```bash
echo 'stxr w0, w0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: unpredictable: identical transfer and status registers
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unpredictable STXR instruction, status is also a source
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88007c00
```
**Verdict:** warning-class-CU

## #164 — encode_ldxr_stxr accepts an X register as STXR status

**Input:** `stxr x0, x1, [x2]`

```bash
echo 'stxr x0, x1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8007c41
```
**Verdict:** silent-accept

## #165 — encode_ldxr_stxr accepts an X data register on byte/half exclusiv

**Input:** `ldxrb x0, [x1]`

```bash
echo 'ldxrb x0, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x085f7c20
```
**Verdict:** silent-accept

## #166 — encode_ldxr_stxr encodes XZR as SP in the address base

**Input:** `ldxr x0, [xzr]`

```bash
echo 'ldxr x0, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85f7fe0
```
**Verdict:** silent-accept

## #167 — encode_logical encodes ANDS on NEON registers

**Input:** `ands v0.8b, v0.8b, v0.8b`

```bash
echo 'ands v0.8b, v0.8b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or predicate register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e201c00
```
**Verdict:** silent-accept

## #168 — encode_logical ignores a trailing extra GPR operand

**Input:** `and w0, w0, w0, w0`

```bash
echo 'and w0, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a000000
```
**Verdict:** silent-accept

## #169 — encode_logical accepts FP/SIMD register names as GPRs

**Input:** `and d0, x0, x0`

```bash
echo 'and d0, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a000000
```
**Verdict:** silent-accept

## #170 — encode_logical accepts mixed X/W register widths

**Input:** `and w0, w0, x0`

```bash
echo 'and w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a000000
```
**Verdict:** silent-accept

## #171 — encode_logical accepts NEON arrangements other than 8b/16b

**Input:** `and v0.4s, v0.4s, v0.4s`

```bash
echo 'and v0.4s, v0.4s, v0.4s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e201c00
```
**Verdict:** silent-accept

## #172 — encode_logical ignores mismatched NEON source arrangements

**Input:** `and v0.16b, v0.8b, v0.16b`

```bash
echo 'and v0.16b, v0.8b, v0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e201c00
```
**Verdict:** silent-accept

## #173 — encode_logical accepts out-of-range shift amounts

**Input:** `and w0, w0, w0, lsl #32`

```bash
echo 'and w0, w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a008000
```
**Verdict:** silent-accept

## #174 — encode_logical accepts SP/WSP in shifted-register form

**Input:** `and wsp, w0, w0`

```bash
echo 'and wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a00001f
cargo test --lib scratch_more -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #175 — encode_logical treats unknown shift kinds as LSL

**Input:** `and w0, w0, w0, lslx #0`

```bash
echo 'and w0, w0, w0, lslx #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register in the immediate operand at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0a000000
```
**Verdict:** silent-accept

## #176 — encode_madd silently ignores a fifth operand

**Input:** `madd w0, w0, w0, w0, x0`

```bash
echo 'madd w0, w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b000000
cargo test --lib scratch_more -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #177 — encode_madd accepts FP/SIMD register names as GPRs

**Input:** `madd d0, x1, x2, x3`

```bash
echo 'madd d0, x1, x2, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b020c20
```
**Verdict:** silent-accept

## #178 — encode_madd accepts mixed X/W register widths

**Input:** `madd w0, w0, w0, x0`

```bash
echo 'madd w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b000000
cargo test --lib scratch_more2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #179 — encode_madd treats SP/WSP as XZR/WZR

**Input:** `madd wsp, w0, w0, w0`

```bash
echo 'madd wsp, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b00001f
```
**Verdict:** silent-accept

## #180 — encode_movk ignores extra operands

**Input:** `movk x0, #0, x0`

```bash
echo 'movk x0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0, 16, 32 or 48
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf2800000
```
**Verdict:** silent-accept

## #181 — encode_movk encodes FP/SIMD register names as GPRs

**Input:** `movk d0, #0`

```bash
echo 'movk d0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x72800000
```
**Verdict:** silent-accept

## #182 — encode_movk silently truncates immediates outside [0, 65535]

**Input:** `movk w0, #-1`

```bash
echo 'movk w0, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 65535].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x729fffe0
```
**Verdict:** silent-accept

## #183 — encode_movk accepts non-lsl shifts and illegal shift amounts

**Input:** `movk w0, #0, lsr #0`

```bash
echo 'movk w0, #0, lsr #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: only 'LSL' shift is permitted at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0 or 16
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x72800000
cargo test --lib scratch_more2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #184 — encode_movk encodes SP/WSP as XZR/WZR

**Input:** `movk wsp, #0`

```bash
echo 'movk wsp, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x7280001f
```
**Verdict:** silent-accept

## #185 — encode_movn ignores extra operands

**Input:** `movn x0, #0, x0`

```bash
echo 'movn x0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0, 16, 32 or 48
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x92800000
cargo test --lib scratch_more2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #186 — encode_movn encodes FP/SIMD register names as GPRs

**Input:** `movn d0, #0`

```bash
echo 'movn d0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x12800000
```
**Verdict:** silent-accept

## #187 — encode_movn silently truncates immediates outside [0, 65535]

**Input:** `movn w0, #-1`

```bash
echo 'movn w0, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 65535].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x129fffe0
cargo test --lib scratch_mov -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #188 — encode_movn accepts non-lsl shifts and illegal shift amounts

**Input:** `movn w0, #0, lsr #0`

```bash
echo 'movn w0, #0, lsr #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: only 'LSL' shift is permitted at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0 or 16
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x12800000
cargo test --lib scratch_mov -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #189 — encode_movn encodes SP/WSP as XZR/WZR

**Input:** `movn wsp, #0`

```bash
echo 'movn wsp, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1280001f
```
**Verdict:** silent-accept

## #190 — encode_movz ignores extra operands after optional lsl

**Input:** `movz x0, #0, x0`

```bash
echo 'movz x0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0, 16, 32 or 48
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd2800000
```
**Verdict:** silent-accept

## #191 — encode_movz encodes FP/SIMD register names as GPRs

**Input:** `movz d0, #0`

```bash
echo 'movz d0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x52800000
cargo test --lib scratch_mov -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #192 — encode_movz silently truncates immediates outside [0, 65535]

**Input:** `movz w0, #-1`

```bash
echo 'movz w0, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 65535].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x529fffe0
```
**Verdict:** silent-accept

## #193 — encode_movz accepts non-lsl shifts and out-of-set lsl amounts

**Input:** `movz w0, #0, lsr #0`

```bash
echo 'movz w0, #0, lsr #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: only 'LSL' shift is permitted at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' with optional integer 0 or 16
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x52800000
```
**Verdict:** silent-accept

## #194 — encode_movz encodes SP/WSP as XZR/WZR

**Input:** `movz wsp, #0`

```bash
echo 'movz wsp, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5280001f
```
**Verdict:** silent-accept

## #195 — encode_neon_qshrn ignores extra operands beyond index 2

**Input:** `sqshrn v0.8b, v0.8h, #1, v0.8b`

```bash
echo 'sqshrn v0.8b, v0.8h, #1, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f9400
cargo test --lib scratch_qshrn -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #196 — encode_neon_qshrn accepts a GPR/FP dest as if it were Vd

**Input:** `sqshrn x0, v0.8h, #1`

```bash
echo 'sqshrn x0, v0.8h, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected register
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
cargo test --lib scratch_qshrn -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** real — parser-masked at CLI (unit-proven: encoder accepts GPR dest)

## #197 — encode_neon_qshrn ignores destination arrangement Tb

**Input:** `sqshrn v0.4h, v0.8h, #1`

```bash
echo 'sqshrn v0.4h, v0.8h, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f9400
```
**Verdict:** silent-accept

## #198 — encode_neon_qshrn truncates i64 shift via `as u32`

**Input:** `sqshrn v0.8h, v0.8b, #4294967297`

```bash
echo 'sqshrn v0.8h, v0.8b, #4294967297' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: qshrn: unsupported source: 8b
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 64 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
```
**Verdict:** real — parser-masked at CLI (unit-proven: as u32 truncates to shift #1)

## #199 — encode_neon_qshrn accepts shift above destination element size

**Input:** `sqshrn v0.8b, v0.8h, #9`

```bash
echo 'sqshrn v0.8b, v0.8h, #9' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 8 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f079400
```
**Verdict:** silent-accept

## #200 — encode_msub silently ignores a fifth operand

**Input:** `msub w0, w0, w0, w0, x0`

```bash
echo 'msub w0, w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b008000
```
**Verdict:** silent-accept

## #201 — encode_msub accepts FP/SIMD register names as GPRs

**Input:** `msub d0, x1, x2, x3`

```bash
echo 'msub d0, x1, x2, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b028c20
```
**Verdict:** silent-accept

## #202 — encode_msub accepts mixed X/W register widths

**Input:** `msub w0, w0, w0, x0`

```bash
echo 'msub w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b008000
```
**Verdict:** silent-accept

## #203 — encode_msub treats SP/WSP as XZR/WZR

**Input:** `msub wsp, w0, w0, w0`

```bash
echo 'msub wsp, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b00801f
```
**Verdict:** silent-accept

## #204 — encode_mul silently ignores a fourth operand

**Input:** `mul w0, w0, w0, x0`

```bash
echo 'mul w0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b007c00
```
**Verdict:** silent-accept

## #205 — encode_mul treats FP/SIMD register names as GPRs

**Input:** `mul d0, x1, x2`

```bash
echo 'mul d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b027c20
```
**Verdict:** silent-accept

## #206 — encode_mul accepts mixed X/W register widths

**Input:** `mul w0, w0, x0`

```bash
echo 'mul w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b007c00
```
**Verdict:** silent-accept

## #207 — encode_mul accepts NEON MUL with 64-bit elements (size==11 UNDEFI

**Input:** `mul v0.2d, v0.2d, v0.2d`

```bash
echo 'mul v0.2d, v0.2d, v0.2d' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4ee09c00
```
**Verdict:** silent-accept

## #208 — encode_mul ignores source NEON arrangements

**Input:** `mul v0.4h, v0.8b, v0.4h`

```bash
echo 'mul v0.4h, v0.8b, v0.4h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e609c00
```
**Verdict:** silent-accept

## #209 — encode_mul encodes SP/WSP as XZR/WZR

**Input:** `mul wsp, w0, w0`

```bash
echo 'mul wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1b007c1f
```
**Verdict:** silent-accept

## #210 — encode_mvn maps unknown shift kinds to LSL

**Input:** `mvn w0, w0, foo #0`

```bash
echo 'mvn w0, w0, foo #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2003e0
```
**Verdict:** silent-accept

## #211 — encode_mvn silently ignores extra non-shift operands

**Input:** `mvn w0, w0, x0`

```bash
echo 'mvn w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2003e0
```
**Verdict:** silent-accept

## #212 — encode_mvn accepts FP/SIMD register names as GPRs

**Input:** `mvn d0, x1`

```bash
echo 'mvn d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2103e0
```
**Verdict:** silent-accept

## #213 — encode_mvn accepts mixed X/W register widths

**Input:** `mvn w0, x0`

```bash
echo 'mvn w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2003e0
```
**Verdict:** silent-accept

## #214 — encode_mvn NEON form silently ignores a third operand

**Input:** `mvn v0.16b, v0.16b, x0`

```bash
echo 'mvn v0.16b, v0.16b, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6e205800
```
**Verdict:** silent-accept

## #215 — encode_mvn ignores source NEON arrangement T

**Input:** `mvn v0.16b, v0.8b`

```bash
echo 'mvn v0.16b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6e205800
```
**Verdict:** silent-accept

## #216 — encode_mvn accepts NEON arrangements other than 8b/16b

**Input:** `mvn v0.4h, v0.4h`

```bash
echo 'mvn v0.4h, v0.4h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e205800
cargo test --lib scratch_mvn -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #217 — encode_mvn accepts out-of-range shift amounts

**Input:** `mvn w0, w0, lsl #32`

```bash
echo 'mvn w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2083e0
```
**Verdict:** silent-accept

## #218 — encode_mvn encodes SP/WSP as XZR/WZR

**Input:** `mvn wsp, w0`

```bash
echo 'mvn wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a2003ff
cargo test --lib scratch_mvn -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #219 — encode_neon_shift_right ignores operands after index 2

**Input:** `srshr v0.8b, v0.8b, #1, v0.8b`

```bash
echo 'srshr v0.8b, v0.8b, #1, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f2400
```
**Verdict:** silent-accept

## #220 — encode_neon_shift_right discards source arrangement

**Input:** `srshr v0.8b, v0.16b, #1`

```bash
echo 'srshr v0.8b, v0.16b, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f2400
```
**Verdict:** silent-accept

## #221 — encode_neon_shift_right accepts a bare GPR/FP/V register as Vn

**Input:** `sshr v0.8b, x0, #1`

```bash
echo 'sshr v0.8b, x0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f0400
```
**Verdict:** silent-accept

## #222 — encode_neon_shift_right truncates i64 shift via `as u32`

**Input:** `sshr v0.8b, v0.8b, #4294967297`

```bash
echo 'sshr v0.8b, v0.8b, #4294967297' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 64 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f0f0400
```
**Verdict:** silent-accept

## #223 — encode_neg silently ignores extra operands

**Input:** `neg w0, w0, w0`

```bash
echo 'neg w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4b0003e0
```
**Verdict:** silent-accept

## #224 — encode_negs maps unknown and ROR shift kinds to LSL

**Input:** `negs w0, w0, ror #0`

```bash
echo 'negs w0, w0, ror #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: 'ROR' operator not allowed at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0003e0
```
**Verdict:** silent-accept

## #225 — encode_negs silently ignores extra non-shift operands

**Input:** `negs w0, w0, x0`

```bash
echo 'negs w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0003e0
```
**Verdict:** silent-accept

## #226 — encode_negs accepts FP/SIMD register names as GPRs

**Input:** `negs d0, x1`

```bash
echo 'negs d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0103e0
```
**Verdict:** silent-accept

## #227 — encode_negs accepts mixed X/W register widths

**Input:** `negs w0, x0`

```bash
echo 'negs w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0003e0
```
**Verdict:** silent-accept

## #228 — encode_negs accepts out-of-range shift amounts

**Input:** `negs w0, w0, lsl #32`

```bash
echo 'negs w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0083e0
```
**Verdict:** silent-accept

## #229 — encode_negs encodes SP/WSP as XZR/WZR

**Input:** `negs wsp, w0`

```bash
echo 'negs wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x6b0003ff
```
**Verdict:** silent-accept

## #230 — encode_neon_shift_imm ignores operands after index 2

**Input:** `ushr v0.8b, v0.8b, #1, v0.8b`

```bash
echo 'ushr v0.8b, v0.8b, #1, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f0400
```
**Verdict:** silent-accept

## #231 — encode_neon_shift_imm discards source arrangement

**Input:** `ushr v0.8b, v0.16b, #1`

```bash
echo 'ushr v0.8b, v0.16b, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f0400
cargo test --lib scratch_shiftimm -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #232 — encode_neon_shift_imm accepts a bare GPR/FP/V register as Vn

**Input:** `ushr v0.8b, x0, #1`

```bash
echo 'ushr v0.8b, x0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f0400
```
**Verdict:** silent-accept

## #233 — encode_neon_shift_imm truncates i64 shift via `as u32`

**Input:** `ushr v0.8b, v0.8b, #4294967297`

```bash
echo 'ushr v0.8b, v0.8b, #4294967297' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 64 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f0400
```
**Verdict:** silent-accept

## #234 — encode_neon_shift_imm panics or encodes out-of-range USHR shift

**Input:** `ushr v0.8b, v0.8b, #-1`

```bash
echo 'ushr v0.8b, v0.8b, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 64 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
cargo test --lib scratch_shiftimm -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** panic-on-invalid

## #235 — encode_neon_tbl accepts a bare Reg as the table list member

**Input:** `tbl v0.8b, {v1.16b}, x0`

```bash
echo 'tbl v0.8b, {v1.16b}, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000020
```
**Verdict:** silent-accept

## #236 — encode_neon_tbl accepts a bare GPR/FP/V as Vm

**Input:** `tbl v0.8b, {v0.16b}, x0`

```bash
echo 'tbl v0.8b, {v0.16b}, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #237 — encode_neon_tbl panics on an empty register list

**Input:** `tbl v0.8b, {}, v0.8b`

```bash
echo 'tbl v0.8b, {}, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: Line 1: empty register list: 'tbl v0.8b, {}, v0.8b'
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: syntax error in register list at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: vector register expected
cargo test --lib scratch_tbl -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** real — parser-masked at CLI (unit-proven: regs[0] panic)

## #238 — encode_neon_tbl ignores extra operands

**Input:** `tbl v0.8b, {v0.16b}, v0.8b, v0.8b`

```bash
echo 'tbl v0.8b, {v0.16b}, v0.8b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #239 — encode_neon_tbl wraps table length above 4 instead of rejecting

**Input:** `tbl v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b`

```bash
echo 'tbl v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: too many registers in vector register list at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid number of vectors
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #240 — encode_neon_tbl accepts a GPR/FP destination

**Input:** `tbl x0, {v0.16b}, v0.8b`

```bash
echo 'tbl x0, {v0.16b}, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #241 — encode_neon_tbl accepts Ta outside {8B,16B}

**Input:** `tbl v0.4h, {v0.16b}, v0.4h`

```bash
echo 'tbl v0.4h, {v0.16b}, v0.4h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #242 — encode_neon_tbl ignores Vm arrangement mismatch with Vd.Ta

**Input:** `tbl v0.8b, {v0.16b}, v0.16b`

```bash
echo 'tbl v0.8b, {v0.16b}, v0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #243 — encode_neon_tbl ignores non-sequential table registers

**Input:** `tbl v0.8b, {v0.16b, v2.16b}, v0.8b`

```bash
echo 'tbl v0.8b, {v0.16b, v2.16b}, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the register list must have a stride of 1 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e002000
```
**Verdict:** silent-accept

## #244 — encode_neon_tbl accepts a table arrangement other than .16B

**Input:** `tbl v0.8b, {v0.8b}, v0.8b`

```bash
echo 'tbl v0.8b, {v0.8b}, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e000000
```
**Verdict:** silent-accept

## #245 — encode_orn ignores a trailing non-shift 4th operand

**Input:** `orn w0, w0, w0, w0`

```bash
echo 'orn w0, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a200000
```
**Verdict:** silent-accept

## #246 — encode_orn encodes FP/SIMD names as GPRs

**Input:** `orn d0, x1, x2`

```bash
echo 'orn d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer, vector or predicate register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a220020
```
**Verdict:** silent-accept

## #247 — encode_orn rejects the GNU ORN-immediate alias

**Input:** `orn w0, w0, #0xaaaaaaaa`

```bash
echo 'orn w0, w0, #0xaaaaaaaa' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: expected register at operand 2, got Some(Imm(2863311530))
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: integer register expected in the extended/shifted operand register at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** not-a-defect (gas parity): llvm-mc-only alias — enhancement request

## #248 — encode_orn accepts NEON ORN arrangements other than 8b/16b

**Input:** `orn v0.8h, v0.8h, v0.8h`

```bash
echo 'orn v0.8h, v0.8h, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ee01c00
```
**Verdict:** silent-accept

## #249 — encode_orn accepts mixed X/W register widths

**Input:** `orn w0, w0, x0`

```bash
echo 'orn w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a200000
```
**Verdict:** silent-accept

## #250 — encode_orn ignores mismatched NEON arrangements and bare Reg sour

**Input:** `orn v0.8b, v0.16b, v0.8b`

```bash
echo 'orn v0.8b, v0.16b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ee01c00
```
**Verdict:** silent-accept

## #251 — encode_orn masks out-of-range shift amounts instead of rejecting 

**Input:** `orn w0, w0, w0, lsl #32`

```bash
echo 'orn w0, w0, w0, lsl #32' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift amount out of range 0 to 31 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a208000
```
**Verdict:** silent-accept

## #252 — encode_orn encodes SP/WSP as XZR/WZR

**Input:** `orn wsp, w0, w0`

```bash
echo 'orn wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer, vector or predicate register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected compatible register or logical immediate
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a20001f
```
**Verdict:** silent-accept

## #253 — encode_orn ignores a trailing operand after a valid shift

**Input:** `orn w0, w0, w0, lsl #1, w0`

```bash
echo 'orn w0, w0, w0, lsl #1, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a200400
```
**Verdict:** silent-accept

## #254 — encode_orn treats unknown shift kinds as LSL

**Input:** `orn w0, w0, w0, lslx #0`

```bash
echo 'orn w0, w0, w0, lslx #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: shift operator expected at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unexpected token in argument list
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a200000
```
**Verdict:** silent-accept

## #255 — encode_ret ignores extra operands

**Input:** `ret x0, x1`

```bash
echo 'ret x0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd65f0000
```
**Verdict:** silent-accept

## #256 — encode_ret accepts FP/SIMD register names as GPRs

**Input:** `ret d0`

```bash
echo 'ret d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd65f0000
```
**Verdict:** silent-accept

## #257 — encode_ret encodes SP as XZR

**Input:** `ret sp`

```bash
echo 'ret sp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd65f03e0
```
**Verdict:** silent-accept

## #258 — encode_ret accepts W-form Rn

**Input:** `ret w0`

```bash
echo 'ret w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd65f0000
```
**Verdict:** silent-accept

## #259 — encode_sbc silently ignores a trailing shift operand

**Input:** `sbc w0, w0, w0, lsl #0`

```bash
echo 'sbc w0, w0, w0, lsl #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a000000
```
**Verdict:** silent-accept

## #260 — encode_sbc accepts FP/SIMD register names as GPRs

**Input:** `sbc d0, x1, x2`

```bash
echo 'sbc d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a020020
cargo test --lib scratch_sbc_smull -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #261 — encode_sbc accepts mixed X/W register widths

**Input:** `sbc w0, w0, x0`

```bash
echo 'sbc w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a000000
```
**Verdict:** silent-accept

## #262 — encode_sbc encodes SP/WSP as XZR/WZR

**Input:** `sbc wsp, w0, w0`

```bash
echo 'sbc wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5a00001f
```
**Verdict:** silent-accept

## #263 — encode_shift accepts size-mismatched and non-GP destinations

**Input:** `shlw $1, %al ; shlq $1, %xmm0`

```bash
echo 'shlw $1, %al ; shlq $1, %xmm0' > probe.s
./target/debug/ccc -c probe.s -o probe_c.o     # ccc expected:  accepts
gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: `%al' not allowed with `shlw'
clang -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x66 d1 e0
```
**Verdict:** silent-accept

## #264 — encode_shift omits FS/GS segment override on memory operands

**Input:** `shlb $0, %fs:(%rax)`

```bash
echo 'shlb $0, %fs:(%rax)' > probe.s
./target/debug/ccc -c probe.s -o probe_c.o     # ccc expected:  accepts
gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc0 20 00
```
**Verdict:** differential

## #265 — encode_shift truncates shift counts outside imm8

**Input:** `shlb $256, %al`

```bash
echo 'shlb $256, %al' > probe.s
./target/debug/ccc -c probe.s -o probe_c.o     # ccc expected:  accepts
gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: /tmp/fullver/p265.s:1: Warning: 0x100 shortened to 0x0
clang -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc0 e0 00
```
**Verdict:** warning-class-CU

## #266 — encode_neon_shll ignores extra operands beyond index 2

**Input:** `sshll v0.8h, v0.8b, #0, v0.8h`

```bash
echo 'sshll v0.8h, v0.8b, #0, v0.8h' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f08a400
```
**Verdict:** silent-accept

## #267 — encode_neon_shll accepts a GPR/FP dest as if it were Vd.Ta

**Input:** `sshll x0, v0.8b, #0`

```bash
echo 'sshll x0, v0.8b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f08a400
```
**Verdict:** silent-accept

## #268 — encode_neon_shll ignores destination arrangement

**Input:** `sshll v0.8b, v0.8b, #0`

```bash
echo 'sshll v0.8b, v0.8b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f08a400
```
**Verdict:** silent-accept

## #269 — encode_neon_shll accepts out-of-range shift (and panics on Imm(-1

**Input:** `sshll v0.8h, v0.8b, #8`

```bash
echo 'sshll v0.8h, v0.8b, #8' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 7 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 7].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f10a400
```
**Verdict:** silent-accept

## #270 — encode_neon_shll does not require Tb to match the 2-suffix / Q

**Input:** `sshll2 v0.8h, v0.8b, #0`

```bash
echo 'sshll2 v0.8h, v0.8b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4f08a400
```
**Verdict:** silent-accept

## #271 — SQSHRUN ignores operands beyond index 2

**Input:** `sqshrun v0.8b, v0.8h, #1, v0.8b`

```bash
echo 'sqshrun v0.8b, v0.8h, #1, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f8400
```
**Verdict:** silent-accept

## #272 — SQSHRUN encodes a GPR/scalar-FP dest as a NEON Rd

**Input:** `sqshrun x0, v0.8h, #1`

```bash
echo 'sqshrun x0, v0.8h, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f8400
```
**Verdict:** silent-accept

## #273 — SQSHRUN ignores destination arrangement Tb

**Input:** `sqshrun v0.4h, v0.8h, #1`

```bash
echo 'sqshrun v0.4h, v0.8h, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f8400
```
**Verdict:** silent-accept

## #274 — SQSHRUN truncates i64 shift immediates with `as u32`

**Input:** `sqshrun v0.8b, v0.8h, #4294967297`

```bash
echo 'sqshrun v0.8b, v0.8h, #4294967297' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 64 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f8400
```
**Verdict:** silent-accept

## #275 — SQSHRUN accepts shift amounts above dest element size

**Input:** `sqshrun v0.8b, v0.8h, #9`

```bash
echo 'sqshrun v0.8b, v0.8h, #9' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 8 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 8].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2f0f8400
```
**Verdict:** silent-accept

## #276 — encode_smull silently ignores a 4th operand

**Input:** `smull x0, w0, w0, x0`

```bash
echo 'smull x0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b207c00
cargo test --lib scratch_sbc_smull -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #277 — encode_smull encodes FP/SIMD register names as GPRs

**Input:** `smull d0, w1, w2`

```bash
echo 'smull d0, w1, w2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b227c20
```
**Verdict:** silent-accept

## #278 — encode_smull encodes SP/WSP as XZR/WZR

**Input:** `smull wsp, w0, w0`

```bash
echo 'smull wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b207c1f
```
**Verdict:** silent-accept

## #279 — encode_smull accepts W dest and X sources

**Input:** `smull w0, w0, w0`

```bash
echo 'smull w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b207c00
```
**Verdict:** silent-accept

## #280 — encode_sxth ignores extra operands

**Input:** `uxtb w0, w0, w0`

```bash
echo 'uxtb w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53001c00
```
**Verdict:** silent-accept

## #281 — encode_sxth encodes FP/SIMD register names as GPRs

**Input:** `sxth d0, w1`

```bash
echo 'sxth d0, w1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13003c20
cargo test --lib scratch_extend -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #282 — encode_sxth encodes SP/WSP as ZR

**Input:** `uxth w0, w0, w0`

```bash
echo 'uxth w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53003c00
```
**Verdict:** silent-accept

## #283 — encode_sxth accepts W dest with X source

**Input:** `sxth w0, x0`

```bash
echo 'sxth w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
```
**Verdict:** silent-accept

## #284 — encode_sxtw ignores extra operands

**Input:** `sxtw x0, w0, x0`

```bash
echo 'sxtw x0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x93407c00
cargo test --lib scratch_extend -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #285 — encode_sxtw accepts FP/SIMD register names as GPRs

**Input:** `sxtb w0, w0, w0`

```bash
echo 'sxtb w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13001c00
```
**Verdict:** silent-accept

## #286 — encode_sxtw encodes SP/WSP as XZR/WZR

**Input:** `sxth w0, w0, w0`

```bash
echo 'sxth w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13003c00
```
**Verdict:** silent-accept

## #287 — encode_sxtw accepts 32-bit dest (Wd)

**Input:** `sxtw w0, x0`

```bash
echo 'sxtw w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
```
**Verdict:** silent-accept

## #288 — encode_neon_shift_left_imm ignores source arrangement

**Input:** `sqshl v0.8b, v0.16b, #0`

```bash
echo 'sqshl v0.8b, v0.16b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f087400
```
**Verdict:** silent-accept

## #289 — encode_neon_shift_left_imm ignores extra operands

**Input:** `sqshl v0.8b, v0.8b, #0, v0.8b`

```bash
echo 'sqshl v0.8b, v0.8b, #0, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f087400
cargo test --lib scratch_shl -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #290 — encode_neon_shift_left_imm accepts non-V register prefixes

**Input:** `sqshl x0.8b, v0.8b, #0`

```bash
echo 'sqshl x0.8b, v0.8b, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f087400
```
**Verdict:** silent-accept

## #291 — encode_neon_shift_left_imm does not reject out-of-range shift

**Input:** `sqshl v0.8b, v0.8b, #-1`

```bash
echo 'sqshl v0.8b, v0.8b, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to add with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 7 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 7].
```
**Verdict:** panic-on-invalid

## #292 — encode_neon_shift_left_imm accepts a source without arrangement

**Input:** `sqshl v0.8b, v0, #0`

```bash
echo 'sqshl v0.8b, v0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0f087400
```
**Verdict:** silent-accept

## #293 — encode_umaddl silently ignores a 5th operand

**Input:** `umaddl x0, w0, w0, x0, x0`

```bash
echo 'umaddl x0, w0, w0, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba00000
```
**Verdict:** silent-accept

## #294 — encode_umaddl encodes FP/SIMD register names as GPRs

**Input:** `umaddl d0, w1, w2, x3`

```bash
echo 'umaddl d0, w1, w2, x3' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba20c20
```
**Verdict:** silent-accept

## #295 — encode_umaddl encodes SP/WSP as XZR/WZR

**Input:** `umaddl wsp, w0, w0, x0`

```bash
echo 'umaddl wsp, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba0001f
```
**Verdict:** silent-accept

## #296 — encode_umaddl accepts W dest, X sources, and W accumulator

**Input:** `umaddl w0, w0, w0, w0`

```bash
echo 'umaddl w0, w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba00000
```
**Verdict:** silent-accept

## #297 — encode_umulh ignores extra operands

**Input:** `umulh x0, x0, x0, x0`

```bash
echo 'umulh x0, x0, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9bc07c00
cargo test --lib scratch_batch5 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #298 — encode_umulh encodes FP/SIMD register names as GPRs

**Input:** `umulh d0, x1, x2`

```bash
echo 'umulh d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9bc27c20
```
**Verdict:** silent-accept

## #299 — encode_umulh encodes SP/WSP as XZR/WZR

**Input:** `umulh wsp, x0, x0`

```bash
echo 'umulh wsp, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9bc07c1f
```
**Verdict:** silent-accept

## #300 — encode_umulh accepts 32-bit W registers

**Input:** `umulh w0, w0, w0`

```bash
echo 'umulh w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9bc07c00
```
**Verdict:** silent-accept

## #301 — encode_neon_rbit accepts a source without arrangement

**Input:** `rbit v0.8b, v0`

```bash
echo 'rbit v0.8b, v0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e605800
```
**Verdict:** silent-accept

## #302 — encode_neon_rbit ignores extra operands

**Input:** `rbit v0.8b, v0.8b, v0.8b`

```bash
echo 'rbit v0.8b, v0.8b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e605800
```
**Verdict:** silent-accept

## #303 — encode_neon_rbit ignores source arrangement

**Input:** `rbit v0.8b, v0.16b`

```bash
echo 'rbit v0.8b, v0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e605800
cargo test --lib scratch_rbit -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #304 — encode_neon_rbit encodes GPR/FP prefixes as V registers

**Input:** `rbit x0.8b, x0.8b`

```bash
echo 'rbit x0.8b, x0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: comma expected between operands at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e605800
cargo test --lib scratch_rbit -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #305 — encode_neon_rbit encodes SP as V31

**Input:** `rbit sp.8b, v0.8b`

```bash
echo 'rbit sp.8b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2e60581f
```
**Verdict:** silent-accept

## #306 — encode_umull silently ignores a 4th operand

**Input:** `umull x0, w0, w0, x0`

```bash
echo 'umull x0, w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba07c00
```
**Verdict:** silent-accept

## #307 — encode_umull encodes FP/SIMD register names as GPRs

**Input:** `umull d0, w1, w2`

```bash
echo 'umull d0, w1, w2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba27c20
```
**Verdict:** silent-accept

## #308 — encode_umull encodes SP/WSP as XZR/WZR

**Input:** `umull wsp, w0, w0`

```bash
echo 'umull wsp, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba07c1f
```
**Verdict:** silent-accept

## #309 — encode_umull accepts W dest and X sources

**Input:** `umull w0, w0, w0`

```bash
echo 'umull w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9ba07c00
```
**Verdict:** silent-accept

## #310 — encode_uxtw ignores extra operands

**Input:** `uxtw x0, w0, x0`

```bash
echo 'uxtw x0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a0003e0
cargo test --lib scratch_batch5 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #311 — encode_uxtw accepts FP/SIMD register names as GPRs

**Input:** `uxtw d0, w1`

```bash
echo 'uxtw d0, w1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a0103e0
cargo test --lib scratch_batch5 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #312 — encode_uxtw emits MOV Wd,Wn (ORR) instead of UBFM Xd,Xn,#0,#31

**Input:** `uxtw x0, w0, w0`

```bash
echo 'uxtw x0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a0003e0
```
**Verdict:** silent-accept

## #313 — encode_uxtw encodes SP/WSP as XZR/WZR

**Input:** `uxtw x0, w0, x0`

```bash
echo 'uxtw x0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a0003e0
```
**Verdict:** silent-accept

## #314 — encode_uxtw accepts 32-bit dest (Wd)

**Input:** `uxtw d0, w0`

```bash
echo 'uxtw d0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2a0003e0
```
**Verdict:** silent-accept

## #315 — encode_ldaxr_stlxr ignores extra operands

**Input:** `stlxr w0, w0, [x0], x2`

```bash
echo 'stlxr w0, w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x8800fc00
```
**Verdict:** silent-accept

## #316 — encode_ldaxr_stlxr encodes SIMD/FP names as GPR Rt

**Input:** `ldaxr d0, [x1]`

```bash
echo 'ldaxr d0, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x885ffc20
```
**Verdict:** silent-accept

## #317 — encode_ldaxr_stlxr ignores a nonzero exclusive offset

**Input:** `stlxr w1, x0, [x2, #-1]`

```bash
echo 'stlxr w1, x0, [x2, #-1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the optional immediate offset can only be 0 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be absent or #0
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc801fc40
```
**Verdict:** silent-accept

## #318 — encode_ldaxr_stlxr encodes SP as ZR in Rt

**Input:** `ldaxr sp, [x0]`

```bash
echo 'ldaxr sp, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85ffc1f
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #319 — encode_ldaxr_stlxr accepts a W register as exclusive base

**Input:** `ldaxr x0, [w1]`

```bash
echo 'ldaxr x0, [w1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85ffc20
```
**Verdict:** silent-accept

## #320 — encode_ldaxr_stlxr encodes STLXR when Ws aliases a source

**Input:** `stlxr w0, w0, [x0]`

```bash
echo 'stlxr w0, w0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: unpredictable: identical transfer and status registers
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unpredictable STXR instruction, status is also a source
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x8800fc00
```
**Verdict:** warning-class-CU

## #321 — encode_ldaxr_stlxr accepts an X register as STLXR status

**Input:** `stlxr x0, x1, [x2]`

```bash
echo 'stlxr x0, x1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc800fc41
```
**Verdict:** silent-accept

## #322 — encode_ldaxr_stlxr accepts Xt on LDAXRB/LDAXRH

**Input:** `ldaxrb x0, [x1]`

```bash
echo 'ldaxrb x0, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x085ffc20
```
**Verdict:** silent-accept

## #323 — encode_ldaxr_stlxr encodes XZR as SP in the exclusive base

**Input:** `ldaxr x0, [xzr]`

```bash
echo 'ldaxr x0, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc85fffe0
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #324 — encode_ldrsw ignores operands beyond the second

**Input:** `ldrsw x0, [x1, #0], x2`

```bash
echo 'ldrsw x0, [x1, #0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: cannot combine pre- and post-indexing at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb9800020
```
**Verdict:** silent-accept

## #325 — encode_ldrsw accepts SIMD/FP destination registers

**Input:** `ldrsw d0, [x1, #0]`

```bash
echo 'ldrsw d0, [x1, #0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb9800020
```
**Verdict:** silent-accept

## #326 — encode_ldrsw rejects LDRSW (literal)

**Input:** `ldrsw x0, foo`

```bash
echo 'ldrsw x0, foo' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: unsupported ldrsw operands: [Reg("x0"), Symbol("foo")]
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** reverse

## #327 — encode_ldrsw silently truncates out-of-range offsets

**Input:** `ldrsw x0, [x1, #-257]`

```bash
echo 'ldrsw x0, [x1, #-257]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate offset out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be an integer in range [-256, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb88ff020
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #328 — encode_ldrsw encodes SP destination as XZR

**Input:** `ldrsw sp, [x1, #0]`

```bash
echo 'ldrsw sp, [x1, #0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb980003f
```
**Verdict:** silent-accept

## #329 — encode_ldrsw accepts a W register as the memory base

**Input:** `ldrsw x0, [w1]`

```bash
echo 'ldrsw x0, [w1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb9800020
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #330 — encode_ldrsw accepts a W register destination

**Input:** `ldrsw w0, [x1, #0]`

```bash
echo 'ldrsw w0, [x1, #0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb9800020
```
**Verdict:** silent-accept

## #331 — encode_ldrsw encodes a W index without UXTW/SXTW as LSL

**Input:** `ldrsw x0, [x1, w2]`

```bash
echo 'ldrsw x0, [x1, w2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of 32-bit register offset at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'uxtw' or 'sxtw' with optional shift of #0 or #2
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb8a26820
```
**Verdict:** silent-accept

## #332 — encode_ldrsw encodes pre/post-index LDRSW when Rt equals Rn

**Input:** `ldrsw x0, [x0, #4]!`

```bash
echo 'ldrsw x0, [x0, #4]!' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: /tmp/fullver/p332.s:1: Warning: unpredictable transfer with writeback
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unpredictable LDR instruction, writeback base is also a source
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb8804c00
```
**Verdict:** warning-class-CU

## #333 — encode_ldrsw encodes XZR/X31 as the memory base SP

**Input:** `ldrsw x0, [xzr]`

```bash
echo 'ldrsw x0, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb98003e0
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #334 — encode_ldtr_sized ignores extra operands

**Input:** `sttrb w0, [x0, #-256], x2`

```bash
echo 'sttrb w0, [x0, #-256], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: cannot combine pre- and post-indexing at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38100800
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #335 — encode_ldtr_sized accepts SIMD/FP registers as Rt

**Input:** `ldtrb d0, [x0]`

```bash
echo 'ldtrb d0, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38400800
```
**Verdict:** silent-accept

## #336 — encode_ldtr_sized silently wraps offsets outside signed 9-bit ran

**Input:** `sttrb w0, [x0, #-257]`

```bash
echo 'sttrb w0, [x0, #-257]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate offset out of range -256 to 255 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be an integer in range [-256, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x380ff800
```
**Verdict:** silent-accept

## #337 — encode_ldtr_sized accepts SP/WSP as Rt

**Input:** `sttrb sp, [x0, #-256]`

```bash
echo 'sttrb sp, [x0, #-256]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3810081f
```
**Verdict:** silent-accept

## #338 — encode_ldtr_sized accepts a W register as the memory base

**Input:** `ldtrb w0, [w0]`

```bash
echo 'ldtrb w0, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38400800
```
**Verdict:** silent-accept

## #339 — encode_ldtr_sized accepts Xt/lr as dest

**Input:** `sttrb x0, [x0, #-256]`

```bash
echo 'sttrb x0, [x0, #-256]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38100800
```
**Verdict:** silent-accept

## #340 — encode_ldtr_sized accepts XZR as the memory base

**Input:** `ldtrb w0, [xzr]`

```bash
echo 'ldtrb w0, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38400be0
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #341 — encode_prfm accepts illegal PRFM (register) shift amounts

**Input:** `prfm pldl1keep, [x0, x1, lsl #1]`

```bash
echo 'prfm pldl1keep, [x0, x1, lsl #1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid shift amount at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' or 'sxtx' with optional shift of #0 or #3
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9217800
```
**Verdict:** silent-accept

## #342 — encode_prfm ignores extra operands

**Input:** `prfm pldl1keep, [x0], x2`

```bash
echo 'prfm pldl1keep, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9800000
```
**Verdict:** silent-accept

## #343 — encode_prfm accepts an FP/SIMD register as the memory base

**Input:** `prfm pldl1keep, [d0]`

```bash
echo 'prfm pldl1keep, [d0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9800000
```
**Verdict:** silent-accept

## #344 — encode_prfm register-offset form uses the wrong opcode bits

**Input:** `prfm pldl1keep, [x0, x1]`

```bash
echo 'prfm pldl1keep, [x0, x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9216800
```
**Verdict:** differential

## #345 — encode_prfm accepts a W register as the memory base

**Input:** `prfm #0, [w0]`

```bash
echo 'prfm #0, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9800000
cargo test --lib scratch_exclusive2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #346 — encode_prfm accepts a W index without UXTW/SXTW

**Input:** `prfm pldl1keep, [x0, w0]`

```bash
echo 'prfm pldl1keep, [x0, w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of 32-bit register offset at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'uxtw' or 'sxtw' with optional shift of #0 or #3
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf9204800
```
**Verdict:** silent-accept

## #347 — encode_prfm accepts XZR/x31 as the memory base

**Input:** `prfm pldl1keep, [xzr]`

```bash
echo 'prfm pldl1keep, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xf98003e0
```
**Verdict:** silent-accept

## #348 — encode_smulh ignores extra operands

**Input:** `smulh x0, x0, x0, x0`

```bash
echo 'smulh x0, x0, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b407c00
cargo test --lib scratch_smulh -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #349 — encode_smulh accepts FP/SIMD registers as GPRs

**Input:** `smulh d0, x1, x2`

```bash
echo 'smulh d0, x1, x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b427c20
```
**Verdict:** silent-accept

## #350 — encode_smulh treats SP/WSP as XZR

**Input:** `smulh wsp, x0, x0`

```bash
echo 'smulh wsp, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or SVE vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b407c1f
```
**Verdict:** silent-accept

## #351 — encode_smulh accepts 32-bit W registers

**Input:** `smulh w0, w0, w0`

```bash
echo 'smulh w0, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b407c00
cargo test --lib scratch_smulh -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #352 — encode_smulh accepts WZR (32-bit form of XZR)

**Input:** `smulh wzr, x0, x0`

```bash
echo 'smulh wzr, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x9b407c1f
```
**Verdict:** silent-accept

## #353 — encode_fcvt_rounding ignores extra operands

**Input:** `fcvtzs w0, s0, x0`

```bash
echo 'fcvtzs w0, s0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate operand required at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [1, 32].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e380000
```
**Verdict:** silent-accept

## #354 — encode_fcvt_rounding encodes FP dest as integer W-form

**Input:** `fcvtzs s0, s0`

```bash
echo 'fcvtzs s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e380000
```
**Verdict:** differential

## #355 — encode_fcvt_rounding encodes H source as ftype=00 (single)

**Input:** `fcvtzs w0, h0`

```bash
echo 'fcvtzs w0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e380000
```
**Verdict:** differential

## #356 — encode_fcvt_rounding encodes SP/WSP dest as ZR

**Input:** `fcvtzs wsp, s0`

```bash
echo 'fcvtzs wsp, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e38001f
```
**Verdict:** silent-accept

## #357 — encode_fp_1src ignores extra operands

**Input:** `fabs s0, s0, s0`

```bash
echo 'fabs s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e20c000
```
**Verdict:** silent-accept

## #358 — encode_fp_1src encodes H registers as ftype=00 (single)

**Input:** `frintn h0, h0`

```bash
echo 'frintn h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e244000
```
**Verdict:** differential

## #359 — encode_fp_1src encodes mixed S/D (and GPR/SP/QVB) instead of reje

**Input:** `fabs s0, d0`

```bash
echo 'fabs s0, d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e20c000
cargo test --lib scratch_fp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #360 — encode_int_to_float ignores extra operands

**Input:** `ucvtf s0, w0, x0`

```bash
echo 'ucvtf s0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate operand required at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e230000
```
**Verdict:** silent-accept

## #361 — encode_int_to_float encodes H dest as ftype=00 (single)

**Input:** `ucvtf h0, w0`

```bash
echo 'ucvtf h0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e230000
cargo test --lib scratch_fp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** differential

## #362 — encode_int_to_float encodes SP/WSP source as ZR

**Input:** `scvtf s0, wsp`

```bash
echo 'scvtf s0, wsp' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e2203e0
cargo test --lib scratch_fp -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #363 — encode_int_to_float encodes GP dest / FP source as integer SCVTF

**Input:** `ucvtf w0, w0`

```bash
echo 'ucvtf w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e230000
```
**Verdict:** silent-accept

## #364 — encode_fcmp treats a missing second operand as FCMP #0.0

**Input:** `fcmp s0`

```bash
echo 'fcmp s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: comma expected between operands at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e202008
```
**Verdict:** silent-accept

## #365 — encode_fcmp ignores a third operand

**Input:** `fcmp s0, s0, s0`

```bash
echo 'fcmp s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e202000
```
**Verdict:** silent-accept

## #366 — encode_fcmp encodes H registers as ftype=00 (single)

**Input:** `fcmp h0, h0`

```bash
echo 'fcmp h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e202000
```
**Verdict:** differential

## #367 — encode_fcmp accepts mixed S/D, GPR, Q/V/B, and SP

**Input:** `fcmp s0, d0`

```bash
echo 'fcmp s0, d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e202000
```
**Verdict:** silent-accept

## #368 — encode_fcvt_precision ignores a third operand

**Input:** `fcvt s0, d0, s0`

```bash
echo 'fcvt s0, d0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e624000
cargo test --lib scratch_fcvt -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #369 — encode_fcvt_precision encodes same-precision FCVT (unallocated)

**Input:** `fcvt s0, s0`

```bash
echo 'fcvt s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e224000
```
**Verdict:** silent-accept

## #370 — encode_fcvt_precision treats SP as an S register

**Input:** `fcvt sp, s0`

```bash
echo 'fcvt sp, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e22401f
```
**Verdict:** silent-accept

## #371 — encode_neon_aes ignores arrangement (accepts T other than .16B)

**Input:** `aese v0.8b, v0.8b`

```bash
echo 'aese v0.8b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e284800
```
**Verdict:** silent-accept

## #372 — encode_neon_aes accepts a bare Vn without arrangement

**Input:** `aese v0.16b, v0`

```bash
echo 'aese v0.16b, v0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e284800
```
**Verdict:** silent-accept

## #373 — encode_neon_aes ignores extra operands

**Input:** `aese v0.16b, v0.16b, v0.16b`

```bash
echo 'aese v0.16b, v0.16b, v0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e284800
```
**Verdict:** silent-accept

## #374 — encode_neon_aes ignores source arrangement

**Input:** `aese v0.16b, v0.8b`

```bash
echo 'aese v0.16b, v0.8b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e284800
```
**Verdict:** silent-accept

## #375 — encode_neon_aes accepts non-V register prefixes

**Input:** `aese x0.16b, x0.16b`

```bash
echo 'aese x0.16b, x0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e284800
cargo test --lib scratch_aes -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #376 — encode_neon_aes encodes SP as V31

**Input:** `aese sp.16b, v0.16b`

```bash
echo 'aese sp.16b, v0.16b' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x4e28481f
```
**Verdict:** silent-accept

## #377 — encode_bfi ignores extra operands

**Input:** `bfi w0, w0, #0, #1, x0`

```bash
echo 'bfi w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic, did you mean: b, bfm, bic, bif, bit, bti, wfi?
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000000
```
**Verdict:** silent-accept

## #378 — encode_bfi accepts FP/SIMD registers as GPR operands

**Input:** `bfi d0, x1, #0, #1`

```bash
echo 'bfi d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000020
```
**Verdict:** silent-accept

## #379 — encode_bfi panics or encodes out-of-range #lsb/#width

**Input:** `bfi w0, w0, #0, #0`

```bash
echo 'bfi w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
cargo test --lib scratch_bfi -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** panic-on-invalid

## #380 — encode_bfi accepts mixed W/X registers

**Input:** `bfi x0, w0, #0, #1`

```bash
echo 'bfi x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb3400000
```
**Verdict:** silent-accept

## #381 — encode_bfi treats SP/WSP as ZR

**Input:** `bfi wsp, w0, #0, #1`

```bash
echo 'bfi wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3300001f
cargo test --lib scratch_bfi -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #382 — encode_bfxil ignores extra operands

**Input:** `bfxil w0, w0, #0, #1, x0`

```bash
echo 'bfxil w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000000
```
**Verdict:** silent-accept

## #383 — encode_bfxil accepts FP/SIMD registers as GPR operands

**Input:** `bfxil d0, x1, #0, #1`

```bash
echo 'bfxil d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000020
```
**Verdict:** silent-accept

## #384 — encode_bfxil panics or encodes out-of-range #lsb/#width

**Input:** `bfxil w0, w0, #0, #0`

```bash
echo 'bfxil w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
```
**Verdict:** panic-on-invalid

## #385 — encode_bfxil accepts mixed W/X registers

**Input:** `bfxil x0, w0, #0, #1`

```bash
echo 'bfxil x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb3400000
```
**Verdict:** silent-accept

## #386 — encode_bfxil treats SP/WSP as ZR

**Input:** `bfxil wsp, w0, #0, #1`

```bash
echo 'bfxil wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3300001f
```
**Verdict:** silent-accept

## #387 — encode_cas accepts X registers for CASB/CASH

**Input:** `casb x0, x1, [x2]`

```bash
echo 'casb x0, x1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x08a07c41
```
**Verdict:** silent-accept

## #388 — encode_cas ignores extra operands

**Input:** `cas w0, w0, [x0], x2`

```bash
echo 'cas w0, w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 3
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88a07c00
```
**Verdict:** silent-accept

## #389 — encode_cas accepts FP/SIMD registers as Rs/Rt

**Input:** `cas s0, s1, [x2]`

```bash
echo 'cas s0, s1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88a07c41
```
**Verdict:** silent-accept

## #390 — encode_cas accepts mixed W/X Rs and Rt

**Input:** `cas x0, w0, [x1]`

```bash
echo 'cas x0, w0, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8a07c20
cargo test --lib scratch_cas -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #391 — encode_cas ignores a nonzero memory offset

**Input:** `cas w0, w0, [x0, #-1]`

```bash
echo 'cas w0, w0, [x0, #-1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the optional immediate offset can only be 0 at operand 3
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88a07c00
```
**Verdict:** silent-accept

## #392 — encode_cas accepts SP/WSP as Rs or Rt

**Input:** `cas sp, w1, [x2]`

```bash
echo 'cas sp, w1, [x2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8bf7c41
cargo test --lib scratch_cas -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #393 — encode_cas accepts a W register or WSP as the memory base

**Input:** `cas w0, w1, [w2]`

```bash
echo 'cas w0, w1, [w2]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 3
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x88a07c41
```
**Verdict:** silent-accept

## #394 — encode_cas accepts XZR/x31/WZR as the memory base

**Input:** `cas x0, x1, [xzr]`

```bash
echo 'cas x0, x1, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.1-a -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 3
clang --target=aarch64-linux-gnu -march=armv8.1-a -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xc8a07fe1
```
**Verdict:** silent-accept

## #395 — encode_cls ignores extra operands

**Input:** `cls w0, w0, x0`

```bash
echo 'cls w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac01400
```
**Verdict:** silent-accept

## #396 — encode_cls accepts FP/SIMD registers as GPRs

**Input:** `cls d0, x1`

```bash
echo 'cls d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac01420
```
**Verdict:** silent-accept

## #397 — encode_cls accepts mixed W/X register widths

**Input:** `cls x0, w0`

```bash
echo 'cls x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac01400
cargo test --lib scratch_cls -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #398 — encode_cls accepts SP/WSP as register 31

**Input:** `cls wsp, w0`

```bash
echo 'cls wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac0141f
```
**Verdict:** silent-accept

## #399 — encode_clz ignores extra operands

**Input:** `clz w0, w0, x0`

```bash
echo 'clz w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac01000
```
**Verdict:** silent-accept

## #400 — encode_clz accepts FP/SIMD registers as scalar CLZ operands

**Input:** `clz d0, x1`

```bash
echo 'clz d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac01020
```
**Verdict:** silent-accept

## #401 — encode_clz accepts mixed W/X register widths

**Input:** `clz x0, w0`

```bash
echo 'clz x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac01000
```
**Verdict:** silent-accept

## #402 — encode_clz accepts SP/WSP as a GPR

**Input:** `clz wsp, w0`

```bash
echo 'clz wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac0101f
```
**Verdict:** silent-accept

## #403 — encode_extr ignores extra operands

**Input:** `extr w0, w0, w0, #0, x0`

```bash
echo 'extr w0, w0, w0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13800000
```
**Verdict:** silent-accept

## #404 — encode_extr accepts FP/SIMD registers as GPRs

**Input:** `extr d0, x1, x2, #0`

```bash
echo 'extr d0, x1, x2, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13820020
```
**Verdict:** silent-accept

## #405 — encode_extr panics or encodes out-of-range #lsb

**Input:** `extr w0, w0, w0, #-1`

```bash
echo 'extr w0, w0, w0, #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 63 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xfffffc00
```
**Verdict:** silent-accept

## #406 — encode_extr accepts mixed W/X registers

**Input:** `extr w0, x0, w0, #0`

```bash
echo 'extr w0, x0, w0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13800000
```
**Verdict:** silent-accept

## #407 — encode_extr accepts SP/WSP as a GPR

**Input:** `extr wsp, w0, w0, #0`

```bash
echo 'extr wsp, w0, w0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1380001f
```
**Verdict:** silent-accept

## #408 — encode_fmov ignores extra operands

**Input:** `fmov s0, s0, s0`

```bash
echo 'fmov s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e204000
```
**Verdict:** silent-accept

## #409 — encode_fmov encodes H registers as ftype=00 (single)

**Input:** `fmov h0, h0`

```bash
echo 'fmov h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e204000
```
**Verdict:** differential

## #410 — encode_fmov encodes SP/WSP as an FP or ZR register

**Input:** `fmov wsp, s0`

```bash
echo 'fmov wsp, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e26001f
```
**Verdict:** silent-accept

## #411 — encode_fmov does not encode FMOV Xd, Vn.D[1] / FMOV Vd.D[1], Xn

**Input:** `fmov x0, v0.d[1]`

```bash
echo 'fmov x0, v0.d[1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  ERROR: fmov needs register operands
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
```
**Verdict:** reverse

## #412 — encode_fmov encodes incompatible register classes instead of reje

**Input:** `fmov q0, s0`

```bash
echo 'fmov q0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e204000
```
**Verdict:** silent-accept

## #413 — encode_fp_arith ignores extra operands

**Input:** `fadd s0, s0, s0, s0`

```bash
echo 'fadd s0, s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e202800
```
**Verdict:** silent-accept

## #414 — encode_fp_arith encodes H registers as ftype=00 (single)

**Input:** `fmul h0, h0, h0`

```bash
echo 'fmul h0, h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e200800
cargo test --lib scratch_farith -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** differential

## #415 — encode_fp_arith encodes mixed S/D (and GPR/SP/QVB) instead of rej

**Input:** `fsub d0, s0, s0`

```bash
echo 'fsub d0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e603800
cargo test --lib scratch_farith2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #416 — encode_rbit ignores extra operands

**Input:** `rbit w0, w0, x0`

```bash
echo 'rbit w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00000
cargo test --lib scratch_rbit_gpr -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #417 — encode_rbit accepts FP/SIMD registers as scalar RBIT operands

**Input:** `rbit d0, x1`

```bash
echo 'rbit d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00020
```
**Verdict:** silent-accept

## #418 — encode_rbit accepts mixed W/X register widths

**Input:** `rbit x0, w0`

```bash
echo 'rbit x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00000
```
**Verdict:** silent-accept

## #419 — encode_rbit accepts SP/WSP as a GPR

**Input:** `rbit wsp, w0`

```bash
echo 'rbit wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac0001f
```
**Verdict:** silent-accept

## #420 — encode_rev16 silently ignores a third operand

**Input:** `rev16 w0, w0, x0`

```bash
echo 'rev16 w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00400
```
**Verdict:** silent-accept

## #421 — encode_rev16 accepts FP/SIMD register names as GPRs

**Input:** `rev16 d0, x1`

```bash
echo 'rev16 d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00420
```
**Verdict:** silent-accept

## #422 — encode_rev16 accepts mixed W/X register widths

**Input:** `rev16 x0, w0`

```bash
echo 'rev16 x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00400
```
**Verdict:** silent-accept

## #423 — encode_rev16 encodes SP/WSP as ZR

**Input:** `rev16 wsp, w0`

```bash
echo 'rev16 wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac0041f
```
**Verdict:** silent-accept

## #424 — encode_rev32 ignores extra operands

**Input:** `rev32 x0, x0, x0`

```bash
echo 'rev32 x0, x0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00800
```
**Verdict:** silent-accept

## #425 — encode_rev32 accepts FP/SIMD registers as GPR operands

**Input:** `rev32 d0, x1`

```bash
echo 'rev32 d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00820
```
**Verdict:** silent-accept

## #426 — encode_rev32 accepts mixed W/X register widths

**Input:** `rev32 x0, w0`

```bash
echo 'rev32 x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00800
cargo test --lib scratch_rev32_sbfiz -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #427 — encode_rev32 NEON path accepts invalid arrangements 2s/4s/2d/1d

**Input:** `rev32 v0.2s, v1.2s`

```bash
echo 'rev32 v0.2s, v1.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x2ea00820
```
**Verdict:** silent-accept

## #428 — encode_rev32 accepts SP/WSP as register 31

**Input:** `rev32 wsp, x0`

```bash
echo 'rev32 wsp, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer register or Advanced SIMD vector register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac0081f
```
**Verdict:** silent-accept

## #429 — encode_rev32 accepts 32-bit W registers

**Input:** `rev32 w0, w0`

```bash
echo 'rev32 w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00800
```
**Verdict:** silent-accept

## #430 — encode_rev silently ignores a third operand

**Input:** `rev w0, w0, x0`

```bash
echo 'rev w0, w0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00800
```
**Verdict:** silent-accept

## #431 — encode_rev accepts FP/SIMD register names as GPRs

**Input:** `rev d0, x1`

```bash
echo 'rev d0, x1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac00820
```
**Verdict:** silent-accept

## #432 — encode_rev accepts mixed W/X register widths

**Input:** `rev x0, w0`

```bash
echo 'rev x0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xdac00c00
```
**Verdict:** silent-accept

## #433 — encode_rev encodes SP/WSP as ZR

**Input:** `rev wsp, w0`

```bash
echo 'rev wsp, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5ac0081f
```
**Verdict:** silent-accept

## #434 — encode_sbfiz silently ignores a 5th operand

**Input:** `sbfiz w0, w0, #0, #1, x0`

```bash
echo 'sbfiz w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000000
```
**Verdict:** silent-accept

## #435 — encode_sbfiz accepts FP/SIMD registers as GPR

**Input:** `sbfiz d0, x1, #0, #1`

```bash
echo 'sbfiz d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000020
cargo test --lib scratch_rev32_sbfiz -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #436 — encode_sbfiz panics or encodes out-of-range #lsb/#width

**Input:** `sbfiz w0, w0, #0, #0`

```bash
echo 'sbfiz w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
```
**Verdict:** panic-on-invalid

## #437 — encode_sbfiz accepts mixed W/X register widths

**Input:** `sbfiz x0, w0, #0, #1`

```bash
echo 'sbfiz x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x93400000
```
**Verdict:** silent-accept

## #438 — encode_sbfiz accepts SP/WSP as register 31

**Input:** `sbfiz wsp, w0, #0, #1`

```bash
echo 'sbfiz wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1300001f
```
**Verdict:** silent-accept

## #439 — encode_ubfiz silently ignores a 5th operand

**Input:** `ubfiz w0, w0, #0, #1, x0`

```bash
echo 'ubfiz w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000000
```
**Verdict:** silent-accept

## #440 — encode_ubfiz accepts FP/SIMD registers as GPR operands

**Input:** `ubfiz d0, x1, #0, #1`

```bash
echo 'ubfiz d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000020
```
**Verdict:** silent-accept

## #441 — encode_ubfiz panics or encodes out-of-range #lsb/#width

**Input:** `ubfiz w0, w0, #0, #0`

```bash
echo 'ubfiz w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
```
**Verdict:** panic-on-invalid

## #442 — encode_ubfiz accepts mixed W/X register widths

**Input:** `ubfiz x0, w0, #0, #1`

```bash
echo 'ubfiz x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd3400000
```
**Verdict:** silent-accept

## #443 — encode_ubfiz accepts SP/WSP as register 31

**Input:** `ubfiz wsp, w0, #0, #1`

```bash
echo 'ubfiz wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5300001f
```
**Verdict:** silent-accept

## #444 — encode_bfm ignores extra operands

**Input:** `bfm w0, w0, #0, #0, x0`

```bash
echo 'bfm w0, w0, #0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000000
cargo test --lib scratch_bfm_family -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #445 — encode_bfm accepts FP/SIMD registers as GPR operands

**Input:** `bfm d0, x1, #0, #0`

```bash
echo 'bfm d0, x1, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x33000020
```
**Verdict:** silent-accept

## #446 — encode_bfm accepts out-of-range immr/imms

**Input:** `bfm w0, w0, #-1, #0`

```bash
echo 'bfm w0, w0, #-1, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 63 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xffff0000
```
**Verdict:** silent-accept

## #447 — encode_bfm accepts mixed W/X registers

**Input:** `bfm x0, w0, #0, #0`

```bash
echo 'bfm x0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb3400000
cargo test --lib scratch_bfm_family -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #448 — encode_bfm treats SP/WSP as ZR

**Input:** `bfm wsp, w0, #0, #0`

```bash
echo 'bfm wsp, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3300001f
```
**Verdict:** silent-accept

## #449 — encode_sbfm ignores extra operands

**Input:** `sbfm w0, w0, #0, #0, x0`

```bash
echo 'sbfm w0, w0, #0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000000
```
**Verdict:** silent-accept

## #450 — encode_sbfm accepts FP/SIMD registers as GPR operands

**Input:** `sbfm d0, x1, #0, #0`

```bash
echo 'sbfm d0, x1, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000020
```
**Verdict:** silent-accept

## #451 — encode_sbfm accepts out-of-range immr/imms

**Input:** `sbfm w0, w0, #-1, #0`

```bash
echo 'sbfm w0, w0, #-1, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 63 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xffff0000
```
**Verdict:** silent-accept

## #452 — encode_sbfm accepts mixed W/X registers

**Input:** `sbfm x0, w0, #0, #0`

```bash
echo 'sbfm x0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x93400000
```
**Verdict:** silent-accept

## #453 — encode_sbfm treats SP/WSP as ZR

**Input:** `sbfm wsp, w0, #0, #0`

```bash
echo 'sbfm wsp, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1300001f
```
**Verdict:** silent-accept

## #454 — encode_sbfx ignores extra operands

**Input:** `sbfx w0, w0, #0, #1, x0`

```bash
echo 'sbfx w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic, did you mean: sbfm?
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000000
```
**Verdict:** silent-accept

## #455 — encode_sbfx accepts FP/SIMD registers as GPR operands

**Input:** `sbfx d0, x1, #0, #1`

```bash
echo 'sbfx d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x13000020
```
**Verdict:** silent-accept

## #456 — encode_sbfx panics or encodes out-of-range #lsb/#width

**Input:** `sbfx w0, w0, #0, #0`

```bash
echo 'sbfx w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
```
**Verdict:** panic-on-invalid

## #457 — encode_sbfx accepts mixed W/X registers

**Input:** `sbfx x0, w0, #0, #1`

```bash
echo 'sbfx x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x93400000
```
**Verdict:** silent-accept

## #458 — encode_sbfx treats SP/WSP as ZR

**Input:** `sbfx wsp, w0, #0, #1`

```bash
echo 'sbfx wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1300001f
cargo test --lib scratch_bfm_family -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #459 — encode_ubfm ignores extra operands

**Input:** `ubfm w0, w0, #0, #0, x0`

```bash
echo 'ubfm w0, w0, #0, #0, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000000
```
**Verdict:** silent-accept

## #460 — encode_ubfm accepts FP/SIMD registers as GPR operands

**Input:** `ubfm d0, x1, #0, #0`

```bash
echo 'ubfm d0, x1, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000020
```
**Verdict:** silent-accept

## #461 — encode_ubfm accepts out-of-range immr/imms

**Input:** `ubfm w0, w0, #-1, #0`

```bash
echo 'ubfm w0, w0, #-1, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 0 to 63 at operand 3
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: immediate must be an integer in range [0, 31].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xffff0000
```
**Verdict:** silent-accept

## #462 — encode_ubfm accepts mixed W/X registers

**Input:** `ubfm x0, w0, #0, #0`

```bash
echo 'ubfm x0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd3400000
```
**Verdict:** silent-accept

## #463 — encode_ubfm accepts SP/WSP as Rd or Rn

**Input:** `ubfm wsp, w0, #0, #0`

```bash
echo 'ubfm wsp, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5300001f
```
**Verdict:** silent-accept

## #464 — encode_ubfx ignores extra operands

**Input:** `ubfx w0, w0, #0, #1, x0`

```bash
echo 'ubfx w0, w0, #0, #1, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unrecognized instruction mnemonic, did you mean: ubfm?
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000000
```
**Verdict:** silent-accept

## #465 — encode_ubfx accepts FP/SIMD registers as GPR operands

**Input:** `ubfx d0, x1, #0, #1`

```bash
echo 'ubfx d0, x1, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x53000020
cargo test --lib scratch_bfm_family -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #466 — encode_ubfx panics or encodes out-of-range #lsb/#width

**Input:** `ubfx w0, w0, #0, #0`

```bash
echo 'ubfx w0, w0, #0, #0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  WARN: panic: attempt to subtract with overflow
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate value out of range 1 to 32 at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected integer in range [1, 32]
```
**Verdict:** panic-on-invalid

## #467 — encode_ubfx accepts mixed W/X registers

**Input:** `ubfx x0, w0, #0, #1`

```bash
echo 'ubfx x0, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xd3400000
```
**Verdict:** silent-accept

## #468 — encode_ubfx treats SP/WSP as ZR

**Input:** `ubfx wsp, w0, #0, #1`

```bash
echo 'ubfx wsp, w0, #0, #1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x5300001f
```
**Verdict:** silent-accept

## #469 — encode_neon_float_two_misc ignores source arrangement

**Input:** `fabs v0.2s, v0.4s`

```bash
echo 'fabs v0.2s, v0.4s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ea0f800
```
**Verdict:** silent-accept

## #470 — encode_neon_float_two_misc accepts a bare source register

**Input:** `fabs v0.2s, v0`

```bash
echo 'fabs v0.2s, v0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of vector register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: too few operands for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ea0f800
```
**Verdict:** silent-accept

## #471 — encode_neon_float_two_misc ignores extra operands

**Input:** `fabs v0.2s, v0.2s, v0.2s`

```bash
echo 'fabs v0.2s, v0.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ea0f800
```
**Verdict:** silent-accept

## #472 — encode_neon_float_two_misc accepts non-V register prefixes

**Input:** `fabs x0.2s, v0.2s`

```bash
echo 'fabs x0.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ea0f800
```
**Verdict:** silent-accept

## #473 — encode_neon_float_two_misc encodes SP/WSP/XZR/WZR/LR as a SIMD re

**Input:** `fabs sp.2s, v0.2s`

```bash
echo 'fabs sp.2s, v0.2s' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected register type at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ea0f81f
```
**Verdict:** silent-accept

## #474 — encode_fabs ignores extra operands

**Input:** `fabs s0, s0, s0`

```bash
echo 'fabs s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e20c000
```
**Verdict:** silent-accept

## #475 — encode_fabs encodes H registers as ftype=00 (single)

**Input:** `fabs h0, h0`

```bash
echo 'fabs h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e20c000
cargo test --lib scratch_h_ftype -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** differential

## #476 — encode_fabs encodes mixed S/D (and GPR/SP/QVB) instead of rejecti

**Input:** `fabs s0, d0`

```bash
echo 'fabs s0, d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e20c000
```
**Verdict:** silent-accept

## #477 — encode_fmadd_fmsub ignores extra operands

**Input:** `fmadd s0, s0, s0, s0, s0`

```bash
echo 'fmadd s0, s0, s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 4
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1f000000
```
**Verdict:** silent-accept

## #478 — encode_fmadd_fmsub encodes H registers as ftype=00 (single)

**Input:** `fmadd h0, h0, h0, h0`

```bash
echo 'fmadd h0, h0, h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1f000000
```
**Verdict:** differential

## #479 — encode_fmadd_fmsub encodes mixed S/D (and GPR/SP/QVB) instead of 

**Input:** `fmadd d0, s0, s0, s0`

```bash
echo 'fmadd d0, s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1f400000
```
**Verdict:** silent-accept

## #480 — encode_fneg ignores extra operands

**Input:** `fneg s0, s0, s0`

```bash
echo 'fneg s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e214000
```
**Verdict:** silent-accept

## #481 — encode_fneg encodes H registers as ftype=00 (single)

**Input:** `fneg h0, h0`

```bash
echo 'fneg h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e214000
cargo test --lib scratch_h_ftype -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** differential

## #482 — encode_fneg encodes mixed S/D (and GPR/SP/QVB) instead of rejecti

**Input:** `fneg s0, d0`

```bash
echo 'fneg s0, d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e214000
```
**Verdict:** silent-accept

## #483 — encode_fsqrt ignores extra operands

**Input:** `fsqrt s0, s0, s0`

```bash
echo 'fsqrt s0, s0, s0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e21c000
```
**Verdict:** silent-accept

## #484 — encode_fsqrt encodes H registers as ftype=00 (single)

**Input:** `fsqrt h0, h0`

```bash
echo 'fsqrt h0, h0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -march=armv8.2-a+fp16 -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -march=armv8.2-a+fp16 -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e21c000
```
**Verdict:** differential

## #485 — encode_fsqrt encodes mixed S/D (and GPR/SP/QVB) instead of reject

**Input:** `fsqrt s0, d0`

```bash
echo 'fsqrt s0, d0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x1e21c000
```
**Verdict:** silent-accept

## #486 — encode_neon_dup ignores extra operands

**Input:** `dup v0.8b, w0, w0`

```bash
echo 'dup v0.8b, w0, w0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: unexpected characters following instruction at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e010c00
```
**Verdict:** silent-accept

## #487 — encode_neon_dup masks out-of-range lane indices

**Input:** `dup v0.8b, v0.b[16]`

```bash
echo 'dup v0.8b, v0.b[16]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: register element index out of range 0 to 15 at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: vector lane must be an integer in range [0, 15].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e010400
```
**Verdict:** silent-accept

## #488 — encode_neon_dup ignores dest arrangement vs element-size mismatch

**Input:** `dup v0.8b, v0.h[0]`

```bash
echo 'dup v0.8b, v0.h[0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e020400
```
**Verdict:** silent-accept

## #489 — encode_neon_dup accepts wrong-width and non-GPR sources

**Input:** `dup v0.8b, x0`

```bash
echo 'dup v0.8b, x0' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: operand mismatch
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0e010c00
cargo test --lib scratch_dup -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #490 — encode_ldrs accepts illegal register-offset shift/extend

**Input:** `ldrb w0, [x0, x0, lsl #1]`

```bash
echo 'ldrb w0, [x0, x0, lsl #1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid shift amount at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'lsl' or 'sxtx' with optional shift of #0
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38607800
```
**Verdict:** silent-accept

## #491 — encode_ldrs ignores operands beyond the second

**Input:** `strb w0, [x0], x2`

```bash
echo 'strb w0, [x0], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be an integer in range [-256, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x39000000
cargo test --lib scratch_ldrs2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #492 — encode_ldrs accepts SIMD/FP destination registers

**Input:** `ldrb d0, [x1, #0]`

```bash
echo 'ldrb d0, [x1, #0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3d400020
```
**Verdict:** silent-accept

## #493 — encode_ldrs truncates out-of-range offsets instead of rejecting t

**Input:** `ldrb w0, [x0, #-257]`

```bash
echo 'ldrb w0, [x0, #-257]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: immediate offset out of range
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: index must be an integer in range [-256, 255].
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x384ff000
```
**Verdict:** silent-accept

## #494 — encode_ldrs accepts SP/WSP as the destination

**Input:** `ldrb sp, [x0]`

```bash
echo 'ldrb sp, [x0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected an integer or zero register at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x3d40001f
cargo test --lib scratch_ldrs2 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #495 — encode_ldrs accepts a W register as the memory base

**Input:** `ldrb w0, [w0, #0]`

```bash
echo 'ldrb w0, [w0, #0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x39400000
```
**Verdict:** silent-accept

## #496 — encode_ldrs accepts a W index without uxtw/sxtw

**Input:** `ldrb w0, [x0, w0]`

```bash
echo 'ldrb w0, [x0, w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid use of 32-bit register offset at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: expected 'uxtw' or 'sxtw' with optional shift of #0
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x38604800
```
**Verdict:** silent-accept

## #497 — encode_ldrs encodes pre/post-index when Rt==Rn (Rn!=SP)

**Input:** `ldr w0, [x0, #4]!`

```bash
echo 'ldr w0, [x0, #4]!' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  WARN: unpredictable transfer with writeback
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: unpredictable LDR instruction, writeback base is also a source
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0xb8404c00
cargo test --lib scratch_ldrs3 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** warning-class-CU

## #498 — encode_ldrs accepts XZR as the memory base

**Input:** `strb w0, [xzr]`

```bash
echo 'strb w0, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x390003e0
cargo test --lib scratch_ldrs3 -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #499 — Illegal post-index immediate is accepted

**Input:** `ld2r {v0.8b, v1.8b}, [x1], #-1`

```bash
echo 'ld2r {v0.8b, v1.8b}, [x1], #-1' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid post-increment amount at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0ddfd020
```
**Verdict:** silent-accept

## #500 — Surplus operand on ld2r/ld3r/ld4r is ignored

**Input:** `ld2r {v0.8b, v1.8b}, [x0], eq`

```bash
echo 'ld2r {v0.8b, v1.8b}, [x0], eq' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: writeback value must be an immediate constant at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d000
```
**Verdict:** silent-accept

## #501 — SIMD/FP register accepted as ldNr base

**Input:** `ld2r {v0.8b, v1.8b}, [d0]`

```bash
echo 'ld2r {v0.8b, v1.8b}, [d0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d000
cargo test --lib scratch_ldnr -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #502 — `[Xn, #imm]` (non-zero unsigned offset) is accepted as no-offset 

**Input:** `ld2r {v0.8b, v1.8b}, [x1, #4]`

```bash
echo 'ld2r {v0.8b, v1.8b}, [x1, #4]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid addressing mode at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d020
```
**Verdict:** silent-accept

## #503 — Non-consecutive / mixed-arrangement register lists are accepted

**Input:** `ld2r {v0.8b, v2.8b}, [x1]`

```bash
echo 'ld2r {v0.8b, v2.8b}, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: the register list must have a stride of 1 at operand 1
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d020
```
**Verdict:** silent-accept

## #504 — Register post-index `[Xn], Xm` is ignored

**Input:** `ld2r {v0.8b, v1.8b}, [x1], x2`

```bash
echo 'ld2r {v0.8b, v1.8b}, [x1], x2' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d020
```
**Verdict:** differential

## #505 — LD2R/LD4R S bit encoded at bit 12 instead of bit 21

**Input:** `ld2r {v0.8b, v1.8b}, [x1]`

```bash
echo 'ld2r {v0.8b, v1.8b}, [x1]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  accepts
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: accepts
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d020
```
**Verdict:** differential

## #506 — W-register base is accepted for ld2r/ld3r/ld4r

**Input:** `ld2r {v0.8b, v1.8b}, [w0]`

```bash
echo 'ld2r {v0.8b, v1.8b}, [w0]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: expected a 64-bit base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d000
```
**Verdict:** silent-accept

## #507 — XZR and x31 are accepted as ldNr base (encoded as SP)

**Input:** `ld2r {v0.8b, v1.8b}, [xzr]`

```bash
echo 'ld2r {v0.8b, v1.8b}, [xzr]' > probe.s
./target/debug/ccc-arm -c probe.s -o probe_c.o     # ccc expected:  accepts
aarch64-linux-gnu-gcc -c probe.s -o probe_g.o 2>&1 | head -2   # gcc expected:  ERROR: invalid base register at operand 2
clang --target=aarch64-linux-gnu -c probe.s -o probe_cl.o 2>&1 | head -2   # clang expected: ERROR: invalid operand for instruction
aarch64-linux-gnu-objdump -d probe_c.o | grep -m1 -E '^[[:space:]]+[0-9a-f]+:'   # ccc encoding: 0x0d40d3e0
cargo test --lib scratch_ldnr -- --nocapture                    # unit witness (expected FAIL pre-fix)
```
**Verdict:** silent-accept

## #508 — PUA encoding silently corrupts U+E080..U+E0FF literals (source-encoding)

```bash
# deterministic witness (expected: test ignored — 'issue #508' witness, pending team decision)
cargo test --lib pbt_tests -- --nocapture | grep -i pua
```
**Verdict:** real — no gcc equivalent (gcc keeps bytes verbatim); see pbt-out/bug_reports/encoding_pua_collision.md

## #509 — ccc-i686 -static silently omits the C runtime when the 32-bit sysroot is absent (linker, i686)

**Input:** `int main(void){return 7;}`

**Precondition (important):** the defect manifests only when the system lacks the i686
sysroot (`/usr/i686-linux-gnu/lib/{crt1.o,libc.a}`). On machines with `gcc-i686-linux-gnu`
installed, ccc links correctly (exit 7) — verified: the apt install of the cross-gcc pulls
in `libc6-dev-i386-cross`, which changed the behavior mid-verification. To reproduce with
the sysroot present, hide it temporarily (`sudo mv /usr/i686-linux-gnu /usr/i686-linux-gnu.bak`)
or use a clean container.

```bash
# with sysroot ABSENT:
./target/debug/ccc-i686 -static /tmp/t7.c -o /tmp/t7
qemu-i386-static /tmp/t7; echo $?            # expected: SIGSEGV (139) — main's ret jumps to argc
readelf -h /tmp/t7 | grep Entry              # expected: entry == main's address; R E segment ≈ 0x10 bytes

i686-linux-gnu-gcc -static /tmp/t7.c -o /tmp/t7_gnu   # reference: links fine on same machine
qemu-i386-static /tmp/t7_gnu; echo $?        # expected: 7 — proper _start + glibc crt
```
**Verdict:** real — conditional (missing-sysroot silent degradation instead of a
"cannot find -lc" diagnostic; GCC hard-errors in the same situation). Filed as issue #509;
exact entry addresses vary by build — check entry==main and the 0x10-byte text segment,
not the absolute address.

## #510 — invalid float↔pointer casts silently accepted; float→ptr = bitcast (frontend, C)

**Input:** `(void*)2147483648.0` / `(double)p` — C11 6.5.4 constraint violations

```bash
gcc /tmp/ptrfloat.c -o /dev/null 2>&1 | head -2   # expected: 'error: cannot convert to a pointer type'
./target/debug/ccc-i686 -S /tmp/i4_f2p.c -o - | grep -cE 'fisttp|cvt'   # expected: 0 — bitcast, no conversion
```
**Verdict:** real — gcc rejects; ccc accepts + reinterprets bits (filed as issue #510)

---
*(503 ARM issues above verified via the ccc-arm/gcc/clang CLI pipeline; these 6 verified individually at unit/E2E level with target-matched references.)*
