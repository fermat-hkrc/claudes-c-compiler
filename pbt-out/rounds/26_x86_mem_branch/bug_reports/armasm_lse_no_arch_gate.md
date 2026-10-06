# AArch64 LSE atomic instructions (ldadd/swp/cas) encoded without checking the +lse architecture feature — both refs reject on default armv8-a
> **Filed as [issue #832](https://github.com/fermat-hkrc/claudes-c-compiler/issues/832)**

# Bug: AArch64 LSE atomic instructions encoded without checking the +lse architecture feature
**Law:** ldadd/swp/cas are ARMv8.1-A LSE extensions; the default AArch64 architecture is armv8-a (no LSE). Both GNU as and llvm-mc reject these forms without `-mattr=+lse`/`-march=armv8.1-a`; the assembler must check the target feature before encoding.
**Impact:** ccc-arm silently encodes LSE instructions regardless of target architecture — the resulting binaries crash on ARMv8.0 hardware (SIGILL). The encodings are CORRECT (byte-identical to llvm-mc with `-mattr=+lse` verified: `410020f8` = ldadd x0,x1,[x2], `418020f8` = swp x0,x1,[x2]) — the bug is the missing feature gate.
**Function:** AArch64 assembler LSE encoder path (no arch-feature check)
**Detected by:** round-26n 24-form atomic/barrier battery (18 clean: ldxr/stxr/ldaxr/stlxr/ldxp/stlxp all byte-exact ✓; dmb/dsb/isb/membar all correct ✓; 6 LSE BUGs)
**Minimal input:** `.text` / `ldadd x0, x1, [x2]` → ccc-arm assembles `410020f8` (correct encoding, wrong arch gate); llvm-mc default rejects, needs `-mattr=+lse`
**Expected:** Err without +lse / -march=armv8.1-a+; Ok with it (and the encoding is correct)
**Actual:** silently encoded regardless of target
**Severity:** low (correct bytes, missing gate — binaries crash only on pre-v8.1 hardware)
**Regression test:** (to add: arch-feature check)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nldadd x0, x1, [x2]\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts!
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # error: instruction requires: lse
```

