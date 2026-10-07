# AArch64 NEON: fcmeq/fcmgt v.4s, #0.0 encodes NOP (comparison silently skipped); fcmlt/fcmle #0.0 wrong encoding; sqshlu/cmls missing
> **Filed as [issue #1057](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1057)**

# Bug: AArch64 NEON FP compare-vs-zero: NOP encoding + wrong encodings + missing mnemonics
**Law:** NEON FP comparisons against #0.0 have dedicated encodings (cond field selects eq/gt/le/lt). Both refs produce exact bytes.
**Impact:** Three classes (45/51 NEON shift+cmp forms byte-exact ✓):
1. **NOP encoding (2 forms):** `fcmeq v0.4s, v1.4s, #0.0` and `fcmgt v0.4s, v1.4s, #0.0` → ccc encodes `1f2003d5` = NOP! The comparison is SILENTLY SKIPPED — vectorized FP equality/ordering checks against zero produce garbage. Same symptom as scalar #839 (fcmp d0,#0.0 → NOP) but in the NEON path.
2. **Wrong encoding (2 forms):** `fcmlt v0.4s, v1.4s, #0.0` → ccc `20d8a04e` (this is fcmeq's encoding!) vs mc `20e8a04e` (correct fcmlt). `fcmle v0.4s, v1.4s, #0.0` → ccc `20d8206e` (bit 23 wrong) vs mc `20d8a06e`.
3. **Missing (2 forms):** `sqshlu` (saturating shift-left-unsigned) and `cmls` (carry-less-set comparison) — both refs accept; ccc rejects.
**Function:** AArch64 assembler NEON FP comparison path
**Detected by:** round-28 NEON shift+cmp battery (51 forms; 45 OK, 2 GAP, 4 DIFF)
**Minimal input:** `.text` / `fcmeq v0.4s, v1.4s, #0.0` → ccc encodes NOP (`1f2003d5`)
**Severity:** HIGH (NOP encoding = silent skip of FP comparison = wrong program behavior)
**Contract evidence:** inferred (double-reference byte comparison)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
fcmeq v0.4s, v1.4s, #0.0
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 1f2003d5 = NOP (WRONG: should be 20d8a04e = fcmeq)
```
