# AArch64 writeback addressing silently accepts offsets beyond the imm9 range — ldr x0, [x1, #504]! encodes garbage
> **Filed as [issue #823](https://github.com/fermat-hkrc/claudes-c-compiler/issues/823)**

# Bug: AArch64 writeback addressing silently accepts offsets beyond the imm9 range

**Law:** Pre/post-index writeback addressing (`[xN, #imm]!` / `[xN], #imm`) uses the **imm9** field (signed 9-bit, **-256..255**), not the scaled imm12 of the plain offset form. An offset outside -256..255 is unencodable in a writeback form and must be rejected.

**Impact:** External `.s` files with writeback offsets in the 256..504 band (legal-looking because the NON-writeback `ldr x0, [x1, #504]` form allows them via imm12 scaling) silently assemble to a DIFFERENT instruction: `ldr x0, [x1, #504]!` encodes `20 8c 5f f8` — the imm9 field takes #504's low bits (504 & 0x1FF = 248), loading from `[x1, #248]` with writeback — the wrong address.

**Function:** AArch64 assembler load/store writeback encoding path

**Detected by:** PBT round-26f writeback boundary battery (byte-level vs llvm-mc; boundary identified between #255 valid and #504 invalid).

**Minimal input:**
```asm
.text
ldr x0, [x1, #504]!
```
```bash
ccc-arm -mlittle-endian -c t.s -o t.o && readelf -x .text t.o
# ccc: assembles, .text = 208c5ff8  (imm9 field garbage)
aarch64-linux-gnu-as t.s -o t.o    # error: 504 is out of range [-256,255]
```

**Expected:** error — imm9 range is [-256, 255]; the plain-offset form must be written without `!`.

**Actual:** silently assembled:
```console
$ readelf -x .text t.o
  0x00000000 208c5ff8
```
Decoding `0xF85F8C20`: imm9 = 0b011111000 = 248 → the instruction is `ldr x0, [x1, #248]!` — the wrong offset.

**Verified boundary:** `ldr x0, [x1, #255]!` is byte-identical to llvm-mc (`20fc4ff8` MATCH); the fault is only the range CHECK for writeback imm9, the encoding field itself is correct within range.

**Severity:** medium (silent wrong-address load — the #776/#792 class on the writeback path)

**Contract evidence:** inferred (double-reference rejection: aarch64-linux-gnu-as AND llvm-mc both reject; ARM ARM defines writeback imm9)

**Fix:** in the writeback (`!`/post-index) path, range-check the offset to [-256, 255] and error outside it; the non-writeback path keeps its imm12 check.

**Regression test:** deterministic: `ldr x0, [x1, #504]!` must be rejected; `ldr x0, [x1, #255]!` must assemble to `20fc4ff8`.
