# AArch64 assembler encodes ALL fmov Rd, #imm (FP immediate) forms as NOP — fmov d0, #1.0 produces 1f2003d5 (NOP) instead of 00106e1e
> **Filed as [issue #1064](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1064)**

# Bug: AArch64 fmov with floating-point immediate encodes as NOP
**Law:** FMOV (immediate) loads a floating-point constant encoded as an 8-bit immediate (imm8 encodes a limited set of FP values). Both refs produce exact encodings.
**Impact:** ALL 10 tested `fmov Rd, #float` forms encode as NOP (`1f2003d5`): fmov d0,#1.0, #2.0, #-1.0, #0.0, #0.125, d1,#3.5, s0,#1.0, #31.0, #0.0, s1,#-2.0. The FP register is NEVER SET — any code relying on fmov-immediate silently gets an unset register. The 28 non-immediate scalar FP forms (fadd/fmul/fdiv/fsqrt/fneg/fabs/fmax/fmin/fcvt/fcvtzs/scvtf/frinti/frinta) all encode correctly. The compiler's own codegen avoids this path (uses integer movq for FP constants), so E2E tests pass — the bug only affects external .s files using fmov-immediate.
**Function:** AArch64 assembler scalar FP immediate encoder
**Detected by:** round-28 scalar FP battery (38 forms; 28 OK, 2 GAP, 10 DIFF — ALL fmov-imm forms NOP)
**Minimal input:** `.text` / `fmov d0, #1.0` → ccc `1f2003d5` (NOP); mc `00106e1e` (fmov d0,#1.0)
**Severity:** HIGH (silent NOP = FP register never set; same pattern as #839 fcmp #0.0 and #1057 fcmeq #0.0)
**Contract evidence:** inferred (double-reference byte comparison)
**Regression test:** (deterministic: fmov d0,#1.0 must encode 00106e1e)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
fmov d0, #1.0
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 1f2003d5 = NOP (WRONG: should be 00106e1e)
```
