# ARM64: FP-bearing small structs ({float}/{float,float}/{double}) passed AND returned in X0/W0 integer registers — AAPCS64 HFA rule requires S/V registers; clang interop reads garbage in all 5 directions
> **Filed as [issue #1118](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1118)**

# Bug: ARM64: FP-bearing small structs ({float}/{float,float}/{double}) passed AND returned in X0/W0 integer registers — AAPCS64 HFA rule requires S/V registers; clang interop reads garbage in all 5 directions
**Law:** ∀ HFA (Homogeneous Floating-point Aggregate, AAPCS64 §6.4.2 — 1-4 FP members, ≤16B): arguments are passed in S/D/V registers and returns delivered in S0/D0/V0. FALSIFIED — ccc-arm uses integer registers (X0/W0 family) for BOTH args and returns; every direction breaks against clang.
**Impact:** Execution-verified interop (clang-21 --target=aarch64-linux-gnu caller + ccc-arm callee, qemu-aarch64-static, exit-code protocol):
```
{float}    arg → BAD      {f,f} ret → BAD
{f,f}      arg → BAD      {d}  ret → BAD
{double}   arg → BAD
(pure-clang control: all OK)
```
Disassembly witnesses (ccc-arm):
- take(struct SF {float v; }) prologue: `str x0, [sp,#16]; ldr w0, [sp,#16]` — arg from W0 (should be S0)
- mk returning {float}: `str w0,[sp,#16]; ldr x0,[sp,#16]` — return via X0 (should be S0)
- RISC-V backend is CORRECT for the same shapes (take reads fa0/fa1 — verified) — defect is ARM-backend-specific.
**Function:** ARM64 call-ABI struct classification — no HFA branch (src/backend/call_abi.rs classifier + ARM codegen param/return paths)
**Detected by:** Differential — round-38 high-value battery ARM extension: freestanding clang interop witness /tmp/arm_m3.c (bit-coded exit, 5 directions), qemu-aarch64-static
**Minimal input:**
```c
struct SF { float v; };
unsigned long long take(struct SF s) { return s.v == 2.5f ? 0xDEADULL : 0xBADULL; }
```
**Expected:** arg in S0 (`fmov s0`-path / `ldr w..` off FP save), AAPCS64 HFA.
**Actual:** arg read from W0 (`str x0,[sp,#16]`); bits happen to interop only when caller ALSO uses integer regs (pure-ccc self-consistent).
**Fix sketch:** classify HFAs in the shared classifier (all-FP members ≤4, size ≤16 → FP class), route ARM64 params/returns through s0-s7/v0-v7 mirroring the RISC-V fa-register handling already present.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
# witness: /tmp/arm_m3.c (bit-coded 5-direction exit) + /tmp/arm_abi2.c (ccc-arm callees)
./target/debug/ccc-arm /tmp/arm_abi2.c -c -o c.o
/usr/lib/llvm-21/bin/clang --target=aarch64-linux-gnu -w -O0 -c /tmp/arm_m3.c -o m.o
/usr/lib/llvm-21/bin/clang --target=aarch64-linux-gnu -nostdlib -static m.o c.o -o x && qemu-aarch64-static ./x; echo $?   # 1 = all five BAD
```
