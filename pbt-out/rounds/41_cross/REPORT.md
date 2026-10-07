# Round 41 — cross-target runtime differential

## Summary
First-ever runtime E2E on non-x86 targets: freestanding rich-C bodies → exit-code protocol → ARM64/RISC-V under qemu + i686 -m32 native, all vs gcc-x64. One link-completeness bug found; codegen itself is clean.

## Bugs Found
### #1145 — long double fails to LINK on ARM64/RISC-V
- **Formal:** ∀ program using long double: linkable binary on every supported target (compiler-emitted helpers must be runtime-provided). FALSIFIED — `undefined symbols: __addtf3, __fixtfdi, __trunctfdf2` on both targets.
- Reproduce: `printf 'int main(void){ long double a=1.0L,b=2.0L; return (int)(a+b); }' | ccc-arm -x c -` → link error.

## Properties
| Property | Cases | Status |
|----------|-------|--------|
| p_cross_target_vs_gcc_x64 (rich bodies: structs/FP/bitfields/unions/switch/loops; ARM64-qemu + RISC-V-qemu + i686-m32) | 150 × 3 targets | **passing** |

## Design Caveats
- ccc-riscv defaults to DYNAMIC linking (interpreter path unrunnable under qemu) — -static required; ccc-arm emits static by default. Behavioral difference, documented in round-41 property (-static flag).
- i686 driver requires main (its crt provides _start); ARM/RISC-V freestanding accept _start. Both paths exercised in the property (dual-source).

## Test Files Created
- src/passes/mod.rs ::pbt_cross_target — resident property (exit-code differential)
