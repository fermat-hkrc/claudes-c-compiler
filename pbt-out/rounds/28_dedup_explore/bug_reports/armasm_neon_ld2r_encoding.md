# AArch64 NEON ld2r/ld4r post-increment byte mismatch + ld1/st1/ld4 writeback (!) on post-index silently accepted
> **Filed as [issue #1052](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1052)**

# Bug: AArch64 NEON ld2r/ld4r wrong encoding + contradictory writeback syntax accepted
**Law:** ld2r/ld4r post-increment has a specific Q/Rm encoding; the `[x0], #N!` (post-inc + writeback !) is contradictory syntax that both refs reject.
**Impact:** Two classes:
1. **BYTE-DIFF (2 forms):** `ld2r {v0.8h, v1.8h}, [x0], #4` — ccc `00d4df4d` vs llvm-mc `00c4ff4d` (wrong Rm field). `ld4r {v0.2d,...}, [x0], #32` — ccc `00fcdf4d` vs mc `00ecff4d` (same wrong field). These are WRONG INSTRUCTIONS, not just size differences.
2. **BUG (3 forms):** `ld1 {v0.16b}, [x0], #16!` / `st1 {v0.16b}, [x0], #16!` / `ld4 {...}, [x0], #64!` — post-index + writeback `!` is contradictory; both refs reject; ccc accepts.
**Function:** AArch64 NEON load/store with post-increment encoding
**Detected by:** round-28 NEON post-inc battery (17 forms; 12 byte-exact, 2 byte-diff, 3 BUG)
**Severity:** high (byte-diffs = wrong instruction encoding)
**Contract evidence:** inferred (double-reference byte comparison)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
ld2r {v0.8h, v1.8h}, [x0], #4
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 00d4df4d (WRONG: should be 00c4ff4d)
```
