# PBT Campaign: ABI return shapes + NaN semantics + -S roundtrip (round 22)

## Scan findings
- Struct-RETURN ABI shapes (all vs gcc at -O0/-O2): 16B II (rax+rdx),
  DD (xmm0+xmm1), MIXED long+double (rax+xmm0), >16B MEMORY via sret,
  {char[16]} (2×INTEGER), {float×4} (one xmm pair) — 6/6 correct.
- NaN/-0.0 semantics: 6/7 agree; signbit(-0.0) folds to 0 on CONSTANT
  operands (globals fine) → #806: irconst_to_bits does `v as i64` instead
  of to_bits() (documented "raw u64 bits for bit-level operations").
- ccc -S → GNU as roundtrip vs ccc -c direct: same code except every
  `movb $imm, %r8` — ccc always 3-byte C6 form, as uses 2-byte B0+r →
  #807 (systematic 1-byte bloat; found because the -S output assembled
  by as is SHORTER than ccc's own object).

## Module: abi_return_deep
- [x] Scan (6 return shapes + NaN battery + roundtrip differential)
- [x] Plan / [x] Test / [x] Review — #806 (medium, documented-and-violated)
  and #807 (low, size) filed; dedup-clean
