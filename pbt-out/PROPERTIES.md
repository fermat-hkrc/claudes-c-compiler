# Properties: encode_fcvt_precision

## encode_fcvt_precision_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FCVT precision decoder). Sibling encode_fcvt_rounding rejected (same-job gate fails: float-to-integer). Sibling encode_neon_fcvtl/fcvtn rejected (vector widen/narrow). RISC-V encode_fcvt_fp rejected (different ISA). Doc evidence: README.md:11 gas-compatible assembler; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:460 fcvt dispatch; ARM ARM FCVT 0 00 11110 ftype 1 0001 opc 10000 Rn Rd; codegen/cast_ops.rs:74-78 emits fcvt d0,s0 / fcvt s0,d0.
- Seed: codegen/cast_ops.rs:74-78; fp_scalar.rs encode_fcmp_pbt / encode_fcvt_rounding_pbt llvm-mc differential
- Formal: ∀ rd,rn ∈ {0..31}, dest_ty,src_ty ∈ {s,d,h} with dest_ty ≠ src_ty, spell ∈ {lower,upper}. encode_fcvt_precision([Reg(spell(dest_ty,rd)), Reg(spell(src_ty,rn))]) = Word(llvm-mc("fcvt dest, src"))
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_ty, src_ty, dest_spell, src_spell]
  domain:
    rd: "0..31"
    rn: "0..31"
    dest_ty: "s|d|h"
    src_ty: "s|d|h with dest_ty != src_ty"
  relation:
    op: eq
    lhs: encode_fcvt_precision([Reg(dest), Reg(src)])
    rhs: llvm_mc_word("fcvt dest, src")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_ty: { gen: int, min: 0, max: 2, type: u32 }
  src_ty: { gen: int, min: 0, max: 2, type: u32 }
  dest_spell: { gen: int, min: 0, max: 1, type: u32 }
  src_spell: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11; encoder/mod.rs:460; ARM ARM FCVT; codegen/cast_ops.rs:74-78
```

## encode_fcvt_precision_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM FCVT field layout on the success path. Stronger differential covers agreement with llvm-mc; this pins the documented bit fields independently. Evidence: fp_scalar.rs:237-239 purpose comment; ARM ARM Floating-point data-processing (1 source) FCVT.
- Seed: fp_scalar.rs encode_fcmp_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, dest_ty ≠ src_ty ∈ {s,d,h}. let w = encode_fcvt_precision([Reg(dest_ty+rd), Reg(src_ty+rn)]). w[31:24]=00011110 ∧ w[23:22]=ftype(src) ∧ w[21]=1 ∧ w[20:17]=0001 ∧ w[16:15]=opc(dest) ∧ w[14:10]=10000 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, dest_ty, src_ty]
  domain:
    rd: "0..31"
    rn: "0..31"
    dest_ty: "s|d|h"
    src_ty: "s|d|h with dest_ty != src_ty"
  relation:
    op: eq
    lhs: encode_fcvt_precision([Reg(dest), Reg(src)])
    rhs: "(0b00011110<<24)|(ftype<<22)|(1<<21)|(0b0001<<17)|(opc<<15)|(0b10000<<10)|(rn<<5)|rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_ty: { gen: int, min: 0, max: 2, type: u32 }
  src_ty: { gen: int, min: 0, max: 2, type: u32 }
evidence: fp_scalar.rs:237-239; ARM ARM FCVT 0 00 11110 ftype 1 0001 opc 10000 Rn Rd
```

## encode_fcvt_precision_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent field increments. Rd+1 / Rn+1 / dest S vs D (src H) / src S vs D (dest H) each flip only the corresponding ARM field. Stronger differential already used; this catches field packing bugs that a single-word equality can miss. Evidence: ARM ARM FCVT field positions.
- Seed: fp_scalar.rs encode_fcmp_metamorphic_fields
- Formal: ∀ rd,rn ∈ {0..30}, dest_ty ≠ src_ty ∈ {s,d,h}. encode(rd+1,rn) = encode(rd,rn) + 1 ∧ encode(rd,rn+1) = encode(rd,rn) + (1<<5) ∧ encode(D,H) XOR encode(S,H) = 1<<15 ∧ encode(H,D) XOR encode(H,S) = 1<<22
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, dest_ty, src_ty]
  domain:
    rd: "0..30"
    rn: "0..30"
    dest_ty: "s|d|h"
    src_ty: "s|d|h with dest_ty != src_ty"
  relation:
    op: holds
    expr: "encode(rd+1)==encode(rd)+1 && encode(rn+1)==encode(rn)+(1<<5) && (encode_D_H ^ encode_S_H)==(1<<15) && (encode_H_D ^ encode_H_S)==(1<<22)"
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  dest_ty: { gen: int, min: 0, max: 2, type: u32 }
  src_ty: { gen: int, min: 0, max: 2, type: u32 }
evidence: ARM ARM FCVT Rd[4:0] Rn[9:5] opc[16:15] ftype[23:22]
```

## encode_fcvt_precision_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects `fcvt s0` / empty as too few operands; SUT documents "fcvt requires 2 operands" at fp_scalar.rs:241-242. Evidence: llvm-mc error; SUT purpose plus assembler gas-compatibility (README.md:11).
- Seed: fp_scalar.rs encode_fcmp_neg_arity
- Formal: ∀ ops with |ops| < 2. encode_fcvt_precision(ops) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n, ty]
  domain:
    len: "0..1"
  relation:
    op: throws
    expr: encode_fcvt_precision(ops_of_len(len))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
  ty: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: fp_scalar.rs:241-242; llvm-mc too few operands; README.md:11
```

## encode_fcvt_precision_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand (`fcvt d0, s1, s2` invalid operand). Gas-compatible assembler must not silently ignore extras. Stronger differential does not cover this invalid domain.
- Seed: fp_scalar.rs encode_fcmp_neg_extra_operand
- Formal: ∀ rd,rn ∈ {0..31}, dest_ty ≠ src_ty ∈ {s,d,h}, extra ∈ Operand. encode_fcvt_precision([Reg(dest), Reg(src), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, dest_ty=0, src_off=1, extra=Reg("s0") → [Reg("s0"), Reg("d0"), Reg("s0")] encodes as 0x1e624000
- Bug report: pbt-out/bug_reports/encode_fcvt_precision_extra_operand.md

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest_ty, src_ty, extra]
  domain:
    dest_ty: "s|d|h"
    src_ty: "s|d|h with dest_ty != src_ty"
  relation:
    op: throws
    expr: encode_fcvt_precision([Reg(dest), Reg(src), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_ty: { gen: int, min: 0, max: 2, type: u32 }
  src_ty: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand on fcvt d0, s1, s2; README.md:11
```

## encode_fcvt_precision_neg_same_precision
- Tier: 4
- Rationale: Negative/error contract. ARM ARM FCVT with ftype==opc is unallocated; llvm-mc rejects `fcvt s0, s1` / `d,d` / `h,h` as invalid operand. Same-precision conversion is not a documented FCVT form. Evidence: ARM ARM unallocated when ftype==opc; llvm-mc.
- Seed: (none) — llvm-mc rejection of same-precision FCVT
- Formal: ∀ rd,rn ∈ {0..31}, ty ∈ {s,d,h}. encode_fcvt_precision([Reg(ty+rd), Reg(ty+rn)]) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, ty=0 → [Reg("s0"), Reg("s0")] encodes as 0x1e224000
- Bug report: pbt-out/bug_reports/encode_fcvt_precision_same_precision.md

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ty]
  domain:
    rd: "0..31"
    rn: "0..31"
    ty: "s|d|h"
  relation:
    op: throws
    expr: encode_fcvt_precision([Reg(ty+rd), Reg(ty+rn)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ty: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: ARM ARM FCVT ftype==opc unallocated; llvm-mc invalid operand
```

## encode_fcvt_precision_neg_wrong_types
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects GPR (x/w), SIMD (q/v/b), and SP/WSP in either slot. FCVT is scalar FP precision conversion among S/D/H only. parse_reg_num maps sp to 31 and `sp` starts with s, so this is a likely SUT hole.
- Seed: fp_scalar.rs encode_fcmp_neg_wrong_types
- Formal: ∀ (a,b) in wrong_type_pairs (GPR, Q/V/B, SP/WSP in either slot, or mixed with a valid S/D/H). encode_fcvt_precision([Reg(a), Reg(b)]) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: (a, b) = ("sp", "s0") — SP dest encoded as S31
- Bug report: pbt-out/bug_reports/encode_fcvt_precision_sp_as_s.md

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [a, b]
  domain:
    a: "GPR or QVB or SP/WSP"
    b: "S/D/H or same invalid class"
  relation:
    op: throws
    expr: encode_fcvt_precision([Reg(a), Reg(b)])
generators:
  a: { gen: string }
  b: { gen: string }
expected_error: String
evidence: llvm-mc rejects fcvt x0,s1 / s0,x1 / q0,s1 / s0,q1 / v0,s1 / sp,s1 / s0,sp / b0,s1 / s0,w1
```

## encode_fcvt_precision_neg_gpr_qvb
- Tier: 4
- Rationale: Sweep — documented unsupported dest/source type arms (fp_scalar.rs:259, 265) for first-char not in {s,d,h}. SP is excluded here because it is a separate failing first-char-'s' hole. llvm-mc rejects x/w/q/v/b/wsp in either slot.
- Seed: fp_scalar.rs encode_fcvt_precision_neg_wrong_types
- Formal: ∀ n,m ∈ {0..31}, which ∈ {0,1}, bad ∈ {Xn,Wn,Qn,Vn,Bn,WSP}, good ∈ {Sn,Dn,Hn}. encode_fcvt_precision(ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, m, which, kind]
  domain:
    which: "0|1"
    bad: "GPR or QVB or WSP"
  relation:
    op: throws
    expr: encode_fcvt_precision(ops_with_slot(which, bad))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: fp_scalar.rs:259,265 unsupported source/dest type; llvm-mc invalid operand
```

## encode_fcvt_precision_neg_nonreg_invalid_name
- Tier: 4
- Rationale: Negative/error contract. Non-register kinds (Imm/Symbol/Label/Mem/Cond/Shift) and invalid names (foo/s32/d32/h32/empty/r0) are not FCVT operands. get_reg and parse_reg_num must Err. Evidence: llvm-mc invalid operand; get_reg expected register.
- Seed: fp_scalar.rs encode_fcmp_neg_nonreg / encode_fcmp_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, bad ∈ nonreg_kinds ∪ invalid_names. encode_fcvt_precision(ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fcvt_precision
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: "0|1"
    bad: "nonreg or invalid name"
  relation:
    op: throws
    expr: encode_fcvt_precision(ops_with_slot(which, bad))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: string }
expected_error: String
evidence: llvm-mc invalid operand; get_reg expected register; parse_reg_num None on foo/s32
```
