# Properties: encode_neon_shift_left_imm

## encode_neon_shift_left_imm_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc. README claims the assembler "accepts the same textual assembly that GCC's gas would consume"; encoder docstring claims 32-bit AArch64 words. State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree SQSHL/UQSHL decoder. encode_neon_shl / encode_neon_sli fail the same-job gate (different opcode/U). SUT-boundary: internal-helper of GNU-style assembler. Mapping: encode_neon_shift_left_imm([Vd.T, Vn.T, #shift], u, 0b01110) <-> `{sqshl|uqshl} Vd.T, Vn.T, #shift`.
- Seed: README.md:228 NEON shifts table; encoder/mod.rs:544-551; llvm-mc KAT `sqshl v0.8b, v1.8b, #0` = 0x0f087420
- Formal: ∀ rd,rn ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1], u ∈ {0,1}. encode_neon_shift_left_imm([Vd.T, Vn.T, Imm(shift)], u, 0b01110) = llvm-mc("{sqshl if u=0 else uqshl} Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 0..(esize(t)-1), u: {0,1} }
  relation:
    op: eq
    lhs: encode_neon_shift_left_imm([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t), Imm(shift)], u, 0b01110)
    rhs: llvm_mc("{sqshl if u=0 else uqshl} v{rd}.{t}, v{rn}.{t}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 0, max: 63, type: i64 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:1-14; encoder/mod.rs:1-7; encoder/mod.rs:544-551; ARM ARM SQSHL/UQSHL (immediate)
```

## encode_neon_shift_left_imm_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Advanced SIMD shift left (immediate) layout `0 Q U 011110 immh:immb opcode 1 Rn Rd` with immh:immb = esize + shift. Stronger differential is a sibling property. Round-trip rejected (no decoder).
- Seed: neon.rs:1741-1768 format comment; ARM ARM C7 SQSHL (immediate)
- Formal: ∀ rd,rn ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1], u ∈ {0,1}, opcode=0b01110. Let w = encode_neon_shift_left_imm(...). Then w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=u ∧ w[28:23]=011110 ∧ w[22:16]=esize(T)+shift ∧ w[15:11]=opcode ∧ w[10]=1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u]
  domain: { rd: 0..31, rn: 0..31, t: valid_T, shift: 0..(esize(t)-1), u: {0,1} }
  body: let w = encode_neon_shift_left_imm([Vd.T,Vn.T,Imm(shift)], u, 0b01110); (w>>31)&1==0 && (w>>30)&1==Q(t) && (w>>29)&1==u && (w>>23)&0x3f==0b011110 && (w>>16)&0x7f==esize(t)+shift && (w>>11)&0x1f==0b01110 && (w>>10)&1==1 && (w>>5)&0x1f==rn && (w&0x1f)==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 0, max: 63, type: i64 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: neon.rs:1741-1768; ARM ARM Advanced SIMD shift left (immediate)
```

## encode_neon_shift_left_imm_metamorphic_u_bit
- Tier: 3
- Rationale: Algebraic metamorphic: SQSHL and UQSHL share the immediate-shift encoding and differ only in U (bit 29). ARM ARM: SQSHL U=0, UQSHL U=1, same opcode 01110. Stronger differential is a sibling. Same-job: both are this helper with different u.
- Seed: encoder/mod.rs:544-551 sqshl u=0 / uqshl u=1; llvm-mc `sqshl v0.8b,v1.8b,#0` XOR `uqshl v0.8b,v1.8b,#0` = 1<<29
- Formal: ∀ rd,rn,T,shift in the valid domain. encode(..., u=0, opcode=0b01110) XOR encode(..., u=1, opcode=0b01110) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: 0..31, rn: 0..31, t: valid_T, shift: 0..(esize(t)-1) }
  relation:
    op: eq
    lhs: encode_neon_shift_left_imm(ops, 0, 0b01110) XOR encode_neon_shift_left_imm(ops, 1, 0b01110)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 0, max: 63, type: i64 }
evidence: encoder/mod.rs:544-551; ARM ARM SQSHL U=0 / UQSHL U=1
```

## encode_neon_shift_left_imm_metamorphic_q_bit
- Tier: 3
- Rationale: Algebraic metamorphic: 64-bit vs 128-bit arrangement pairs of the same esize (8b/16b, 4h/8h, 2s/4s) differ only in Q (bit 30). ARM ARM Q selects arrangement width. 2d has no Q=0 pair (1d is unallocated).
- Seed: neon.rs:1752-1759 Q from arrangement; neighbouring encode_neon_sli_pbt Q-bit property
- Formal: ∀ rd,rn, (Tlo,Thi) ∈ {(8b,16b),(4h,8h),(2s,4s)}, shift ∈ [0, esize(Tlo)-1]. encode(Tlo) XOR encode(Thi) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, tlo, thi, shift, u]
  domain: { (tlo,thi): {(8b,16b),(4h,8h),(2s,4s)}, shift: 0..(esize(tlo)-1), u: {0,1} }
  relation:
    op: eq
    lhs: encode(Tlo) XOR encode(Thi)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tlo_thi: { gen: oneof, items: [["8b","16b"], ["4h","8h"], ["2s","4s"]] }
  shift: { gen: int, min: 0, max: 31, type: i64 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: neon.rs:1752-1759; ARM ARM Q bit
```

## encode_neon_shift_left_imm_metamorphic_shift_inc
- Tier: 3
- Rationale: Algebraic metamorphic: immh:immb = esize + shift, so incrementing shift by 1 (while still in range) adds 1 to bits [22:16] and leaves all other bits unchanged. ARM ARM shift encoding. Bounds pinned at 0 and esize-2.
- Seed: neon.rs:1761-1768 immh:immb = esize + shift; neighbouring encode_neon_sli_pbt shift_inc
- Formal: ∀ rd,rn,T,shift ∈ [0, esize(T)-2], u ∈ {0,1}. encode(shift+1) - encode(shift) = 1<<16
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u]
  domain: { t: valid_T, shift: 0..(esize(t)-2), u: {0,1} }
  relation:
    op: eq
    lhs: encode(shift+1) - encode(shift)
    rhs: 1 << 16
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 0, max: 62, type: i64 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: neon.rs:1761-1768; ARM ARM immh:immb = esize + shift
```

## encode_neon_shift_left_imm_neg_extra_operands
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 4th operand ("invalid operand for instruction"); GNU-style SQSHL/UQSHL immediate takes exactly 3 operands. README "same textual assembly that GCC's gas would consume". Stronger oracles do not cover the extra-operand error path.
- Seed: llvm-mc `sqshl v0.8b, v1.8b, #0, v2.8b` error; neighbouring encode_neon_sli_pbt neg_extra_operands
- Formal: ∀ valid (rd,rn,T,shift,u), extra ∈ Operand. encode([Vd.T, Vn.T, Imm(shift), extra], u, 0b01110) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b", shift=0, u=0, extra_kind=0 (sqshl v0.8b, v0.8b, #0, v0.8b)
- Bug report: pbt-out/bug_reports/encode_neon_shift_left_imm_extra_operand.md

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u, extra]
  domain: { valid 3-operand SQSHL/UQSHL plus a 4th Operand }
  relation:
    op: throws
    expr: encode_neon_shift_left_imm([Vd.T, Vn.T, Imm(shift), extra], u, 0b01110)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc rejects extra operand; README.md:1-14 GNU-style gas compatibility
```

## encode_neon_shift_left_imm_neg_shift_oob
- Tier: 4
- Rationale: Negative/error contract. ARM ARM and llvm-mc require shift ∈ [0, esize-1] (`immediate must be an integer in range [0, 7]` for 8b). Bounds pinned at -1, esize, esize+1. i64 values that truncate via `as u32` are included. Stronger differential does not cover the rejection path.
- Seed: llvm-mc `sqshl v0.8b, v1.8b, #8` and `#-1` error; neon.rs:1761-1766 documented ranges
- Formal: ∀ rd,rn,T, u, shift ∉ [0, esize(T)-1]. encode(..., Imm(shift), ...) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=-1, u=0 (debug panic: attempt to add with overflow at neon.rs:1768)
- Bug report: pbt-out/bug_reports/encode_neon_shift_left_imm_shift_oob.md

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u]
  domain: { shift < 0 or shift >= esize(t) }
  relation:
    op: throws
    expr: encode_neon_shift_left_imm([Vd.T, Vn.T, Imm(shift)], u, 0b01110)
expected_error: String
generators:
  shift: { gen: int, min: -16, max: 256, type: i64 }
evidence: llvm-mc shift range; ARM ARM SQSHL (immediate) shift in [0, esize-1]; neon.rs:1761-1766
```

## encode_neon_shift_left_imm_neg_arity_shape
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects arity < 3, unsupported T (1d, 8s, empty), dest/src T mismatch, non-RegArrangement slots, invalid register names, non-Imm shift. GNU-style syntax is Vd.T, Vn.T, #shift with matching T.
- Seed: llvm-mc `sqshl v0.8b, v1.8b` too few operands; `sqshl v0.8b, v1.16b, #0` invalid operand; `sqshl v0.1d, v1.1d, #0` invalid; neighbouring encode_neon_sli_pbt neg_arity_and_shape / neg_arrangement_mismatch
- Formal: ∀ inputs with len<3 ∨ T ∉ valid ∨ destT≠srcT ∨ non-arrangement register ∨ invalid name ∨ non-Imm shift. encode is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: dest T=8b, src T=16b, shift=0, u=0 (sqshl v0.8b, v0.16b, #0); arity-0 / T=1d / v32 still Err as required
- Bug report: pbt-out/bug_reports/encode_neon_shift_left_imm_arrangement_mismatch.md

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, u]
  domain: { arity 0..2, unsupported T, mismatched T, non-reg, invalid name, non-Imm }
  relation:
    op: throws
    expr: encode_neon_shift_left_imm(ops, u, 0b01110)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  tbad: { gen: oneof, items: ["1d", "8s", "1s", "3s", "", "b", "h"] }
evidence: llvm-mc rejects these shapes; README.md:1-14; neon.rs:1745-1759
```

## encode_neon_shift_left_imm_neg_non_v_prefix
- Tier: 4
- Rationale: Negative/error contract (coverage sweep). llvm-mc rejects GPR/scalar prefixes (`x0.8b`, `d0.8b`, …). GNU-style SQSHL/UQSHL vector form requires V registers. parse_reg_num accepts x/w/d/s/q/h/b, so a missing V-class check would silently encode the number.
- Seed: llvm-mc `sqshl x0.8b, v1.8b, #0` error; neighbouring encode_neon_sli_pbt neg_non_v_prefix
- Formal: ∀ rd,rn,T,shift,u, prefix ∈ {x,w,d,s,q,h,b}, which ∈ {dest,src}. encode with register name `{prefix}{rd}` is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=0, u=0, prefix="x", which=0 (sqshl x0.8b, v0.8b, #0)
- Bug report: pbt-out/bug_reports/encode_neon_shift_left_imm_non_v_prefix.md

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u, prefix, which]
  domain: { prefix: {x,w,d,s,q,h,b}, which: {dest, src} }
  relation:
    op: throws
    expr: encode_neon_shift_left_imm(ops_with_non_v_prefix, u, 0b01110)
expected_error: String
generators:
  prefix: { gen: oneof, items: ["x", "w", "d", "s", "q", "h", "b"] }
evidence: llvm-mc rejects non-V prefix; README.md:1-14 GNU-style gas compatibility
```

## encode_neon_shift_left_imm_neg_reg_no_arrangement
- Tier: 4
- Rationale: Negative/error contract (coverage sweep). llvm-mc rejects missing arrangement (`sqshl v0, v1.8b, #0` / `sqshl v0.8b, v1, #0`). get_neon_reg accepts Operand::Reg and returns an empty arrangement string.
- Seed: llvm-mc missing-arrangement error; get_neon_reg neon.rs:14-17 Operand::Reg branch
- Formal: ∀ rd,rn,T,shift,u. encode with dest or src as Operand::Reg (no arrangement) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=0, u=0, which=1 (sqshl v0.8b, v0, #0)
- Bug report: pbt-out/bug_reports/encode_neon_shift_left_imm_src_reg_no_arrangement.md

```property
function: encoder.neon.encode_neon_shift_left_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u, which]
  domain: { which: {dest, src} }
  relation:
    op: throws
    expr: encode_neon_shift_left_imm(ops_with_Reg_slot, u, 0b01110)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: llvm-mc rejects missing arrangement; neon.rs:14-17 get_neon_reg Operand::Reg
```
