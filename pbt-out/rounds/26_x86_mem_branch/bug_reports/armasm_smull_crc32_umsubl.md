# AArch64: smull accepts 64-bit sources (wrong encoding); CRC32 family encoded without +crc gate; umsubl/smnegl missing
> **Filed as [issue #837](https://github.com/fermat-hkrc/claudes-c-compiler/issues/837)**

# Bug: AArch64 smull width violation, CRC32 without arch gate, umsubl/smnegl missing
**Law:** smull's sources must be W (32-bit) registers only; CRC32 requires the +crc extension; umsubl/smnegl are standard mnemonics.
**Impact:** Three classes found in the 19-form multiply-add battery (14 clean byte-exact ✓):
1. `smull x0, x1, x2` (both sources X = 64-bit) silently accepted — encodes `207c229b` = smull x0,w1,w2 — the X sources are silently treated as W (truncation without error). Both refs reject.
2. `crc32b/crc32h/crc32w/crc32cx` — 4 forms silently encoded without the +crc gate. Encodings ARE correct (crc32b `2040c21a` matches llvm-mc with `+crc`). Same class as #832 (LSE) / #833 (RISC-V atomics).
3. `umsubl x0, w1, w2, x3` and `smnegl x0, w1, w2` — both refs accept; ccc rejects (missing mnemonics, same class as #830's lslv/rorv).
**Function:** AArch64 assembler smull path + CRC32 encoder + mnemonic table
**Detected by:** round-26t 19-form mul-add battery (14 byte-exact; 3 BUG + 2 GAP classes)
**Minimal input:** `.text` / `smull x0, x1, x2` → ccc-arm assembles `207c229b`; both refs reject (64-bit sources on a W-source instruction)
**Expected:** Err for smull X-src; Err for CRC32 without +crc; Ok for umsubl/smnegl
**Actual:** silent accept (smull truncates to W), silent encode (CRC32), reject (umsubl/smnegl)
**Severity:** medium (smull silent truncation) + low (CRC32 gate) + low (missing mnemonics)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nsmull x0, x1, x2\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts → 207c229b
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # Error: operand mismatch
```

