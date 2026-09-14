# Properties: encode_neon_dup

## encode_neon_dup_diff_general
- Tier: 2
- Rationale: Differential vs llvm-mc is the strongest evidenced oracle for the GPR form. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree DUP decoder; UMOV is a different opcode). Sibling encode_neon_umov / encode_neon_ins rejected (same-job gate: UMOV opcode 001111, INS 000111). Doc evidence: README.md:12 GNU-style assembly contract; README.md:234 lists dup (element/GPR); encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:670 dup dispatch; ARM ARM Advanced SIMD DUP (general).
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_diff_llvm_mc
- Formal: ∀ rd,rn ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s,2d}, spell ∈ GPR spelling for T (Wn including wzr for T≠2d; Xn including xzr for T=2d). encode_neon_dup([RegArrangement(v{rd},T), Reg(spell(rn))]) = Word(w) ∧ llvm-mc("dup v{rd}.{T}, " ++ spell(rn)) = w
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b,4h,8h,2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_neon_dup([RegArrangement(v{rd}, t), Reg(gpr_for(t, rn))])
    rhs: llvm_mc_word("dup v{rd}.{t}, " ++ gpr_for(t, rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
evidence: README.md:12 GNU-style assembly; README.md:234 dup (element/GPR); encoder/mod.rs:670 dup dispatch; ARM ARM DUP (general); llvm-mc -triple=aarch64 -show-encoding
```

## encode_neon_dup_diff_element
- Tier: 2
- Rationale: Differential vs llvm-mc for the element form. Same stronger-oracle rejection as encode_neon_dup_diff_general. Dest T must match element size; index in ARM range. Doc evidence: README.md:234 dup (element/GPR); neon.rs:511-527 purpose comment; ARM ARM Advanced SIMD DUP (element).
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_diff_llvm_mc
- Formal: ∀ rd,rn ∈ [0,31], (T,Ts) ∈ {(8b,b),(16b,b),(4h,h),(8h,h),(2s,s),(4s,s),(2d,d)}, i ∈ [0, imax(Ts)]. encode_neon_dup([RegArrangement(v{rd},T), RegLane(v{rn},Ts,i)]) = Word(w) ∧ llvm-mc("dup v{rd}.{T}, v{rn}.{Ts}[{i}]") = w
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, ts, i]
  domain: { rd: 0..31, rn: 0..31, (t,ts): matching pairs, i: 0..imax(ts) }
  relation:
    op: eq
    lhs: encode_neon_dup([RegArrangement(v{rd}, t), RegLane(v{rn}, ts, i)])
    rhs: llvm_mc_word("dup v{rd}.{t}, v{rn}.{ts}[{i}]")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: README.md:234 dup (element/GPR); neon.rs:511-527 element form; ARM ARM DUP (element); llvm-mc -triple=aarch64 -show-encoding
```

## encode_neon_dup_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM DUP field layout, independent of llvm-mc. Stronger differential already owned by the two diff properties. Round-trip rejected (no decoder). Doc evidence: ARM ARM Advanced SIMD DUP (general/element); purpose comments neon.rs:496-527 name the ARM layout (not the producing assignment).
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_arm_fields
- Formal: ∀ rd,rn ∈ [0,31], T ∈ valid. encode_neon_dup(GPR form) = Word(w) ⇒ bit31=0 ∧ Q=q(T) ∧ bits[29:21]=001110000 ∧ imm5=imm5_size(T) ∧ bits[15:10]=000011 ∧ Rn=rn ∧ Rd=rd. Element form same with bits[15:10]=000001 and imm5=imm5_index(Ts,i).
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, form]
  domain: { rd: 0..31, rn: 0..31, t: valid T, form: {gpr, element} }
  body: bit31(w)==0 && Q(w)==q(t) && bits[29:21]==0b001110000 && imm5(w)==expected_imm5 && bits[15:10]==opc && Rn==rn && Rd==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
evidence: ARM ARM DUP (general) 0 Q 0 01110 000 imm5 000011 Rn Rd; DUP (element) opcode 000001; neon.rs:496-527 purpose comments
```

## encode_neon_dup_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent Rd/Rn/Q/opcode fields. Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; 8b vs 16b (4h vs 8h, 2s vs 4s) flips only Q bit 30; GPR vs element form flips only bit 11. ARM ARM DUP layout. Weaker than differential; required metamorphic angle.
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ [0,30], T_lo ∈ {8b,4h,2s}. let w = encode_neon_dup(GPR, rd, rn, T_lo). encode(rd+1)=w+1 ∧ encode(rn+1)=w+(1<<5) ∧ encode(T_hi) XOR w = (1<<30) ∧ encode(element i=0) XOR w = (1<<11)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t_lo]
  domain: { rd: 0..30, rn: 0..30, t_lo: {8b,4h,2s} }
  body: w_rd == w + 1 && w_rn == w + (1 << 5) && (w_q ^ w) == (1u32 << 30) && (w_elem ^ w) == (1u32 << 11)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  t_lo: { gen: oneof, values: ["8b", "4h", "2s"] }
evidence: ARM ARM DUP Q at bit 30, Rd[4:0], Rn[9:5], general opcode bit11=1 vs element bit11=0
```

## encode_neon_dup_neg_arity_invalid
- Tier: 5
- Rationale: Negative/error contract. DUP requires 2 operands (neon.rs:492-494 "dup requires 2 operands"); invalid register names fail parse_reg_num (mod.rs:131-147); unsupported T fails neon_arr_to_q_size / arrangement match. llvm-mc rejects these. Stronger oracles do not cover the error domain.
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_neg_arity
- Formal: ∀ ops. |ops|<2 ∨ dest/src invalid name ∈ {foo,v32,v,empty,v-1,v99} ∨ T ∉ {8b,16b,4h,8h,2s,4s,2d} ∨ dest/src non-register kind ⇒ encode_neon_dup(ops) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, bad, t_bad, which]
  domain: { n: 0..1, bad: invalid names, t_bad: unsupported T }
  relation:
    op: throws
    expr: encode_neon_dup(short_or_invalid)
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  which: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: neon.rs:492-494 dup requires 2 operands; neon.rs:532 unsupported dup arrangement; parse_reg_num None; llvm-mc rejects
```

## encode_neon_dup_neg_extra_operands
- Tier: 5
- Rationale: Negative/error contract. GNU-style DUP is a 2-operand instruction; llvm-mc rejects a 3rd operand. README.md:12 same textual assembly as gas. encode_neon_dup currently checks only len<2 (producing statement is not evidence that extras are allowed).
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_neg_extra
- Formal: ∀ valid 2-operand DUP ops, extra ∈ Operand. encode_neon_dup(ops ++ [extra]) = Err ∧ llvm-mc(asm with 3rd operand) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", extra_kind=0 — dup v0.8b, w0, w0
- Bug report: pbt-out/bug_reports/encode_neon_dup_extra_operand.md

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, extra]
  domain: { rd: 0..31, rn: 0..31, t: valid T }
  relation:
    op: throws
    expr: encode_neon_dup(valid ++ [extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: README.md:12 GNU-style assembly; llvm-mc rejects `dup v0.4s, w1, w2`; ARM ARM DUP is 2-operand
```

## encode_neon_dup_neg_index_oor
- Tier: 5
- Rationale: Negative/error contract. ARM ARM DUP (element) index ranges: b 0-15, h 0-7, s 0-3, d 0-1. llvm-mc rejects OOR index. SUT masks index (`index & 0xF` etc.) — producing statement is not evidence that wrapping is allowed.
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_neg_arrangement_mismatch
- Formal: ∀ rd,rn ∈ [0,31], T ∈ valid, i > imax(elem_of(T)). encode_neon_dup([RegArrangement(v{rd},T), RegLane(v{rn},elem_of(T),i)]) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", i_extra=1 — dup v0.8b, v0.b[16]
- Bug report: pbt-out/bug_reports/encode_neon_dup_index_oor.md

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, i]
  domain: { i > imax(elem_of(t)) }
  relation:
    op: throws
    expr: encode_neon_dup([RegArrangement(v{rd}, t), RegLane(v{rn}, elem_of(t), i)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  i: { gen: int, min: 1, max: 8, type: u32 }
expected_error: String
evidence: ARM ARM DUP (element) index ranges; llvm-mc `vector lane must be an integer in range`
```

## encode_neon_dup_neg_size_mismatch
- Tier: 5
- Rationale: Negative/error contract. Dest T must match element size (8b/16b↔b, 4h/8h↔h, 2s/4s↔s, 2d↔d). llvm-mc rejects size mismatch. SUT uses dest T only for Q and elem_size independently for imm5 — producing statement is not evidence that mismatch is allowed.
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_neg_arrangement_mismatch
- Formal: ∀ rd,rn ∈ [0,31], T ∈ valid, Ts ≠ elem_of(T), i ∈ [0, imax(Ts)]. encode_neon_dup([RegArrangement(v{rd},T), RegLane(v{rn},Ts,i)]) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", ts_other="h", i_mis=0 — dup v0.8b, v0.h[0]
- Bug report: pbt-out/bug_reports/encode_neon_dup_size_mismatch.md

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, ts, i]
  domain: { size(t) != ts }
  relation:
    op: throws
    expr: encode_neon_dup([RegArrangement(v{rd}, t), RegLane(v{rn}, ts, i)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  i: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: ARM ARM DUP (element) dest T matches element size; llvm-mc rejects `dup v0.8b, v0.h[0]`
```

## encode_neon_dup_neg_gpr_width
- Tier: 5
- Rationale: Negative/error contract. ARM ARM DUP (general) source is Wn for 8/16/32-bit T and Xn for 64-bit T (.2d). llvm-mc rejects X on 32-bit T, W on .2d, SP/WSP, and FP/SIMD names as the GPR source. parse_reg_num accepting any prefix is not a license to encode a wrong-width or non-GPR source.
- Seed: neon.rs encode_neon_float_two_misc_pbt encode_neon_float_two_misc_neg_sp
- Formal: ∀ rd ∈ [0,31], T ∈ valid, bad_src ∈ wrong-width GPR ∪ {sp,wsp,d0,s0,q0,v0,h0,b0} (and lr when T≠2d, xzr when T≠2d, wzr when T=2d). encode_neon_dup([RegArrangement(v{rd},T), Reg(bad_src)]) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, t="8b", n=0, alias="sp" — dup v0.8b, x0
- Bug report: pbt-out/bug_reports/encode_neon_dup_wrong_width_gpr.md

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, bad_src]
  domain: { rd: 0..31, t: valid T, bad_src: wrong-width or non-GPR }
  relation:
    op: throws
    expr: encode_neon_dup([RegArrangement(v{rd}, t), Reg(bad_src)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: ARM ARM DUP (general) Rn is W for T!=2D and X for T=2D; llvm-mc rejects `dup v0.2d, w1` and `dup v0.4s, x1` and `dup v0.4s, sp`
```

## encode_neon_dup_neg_elem_invalid
- Tier: 5
- Rationale: Coverage sweep (round 1/1). coverage_gaps had no LLVM profraw; manual arm audit of encode_neon_dup showed untested element-form parse_reg_num None (neon.rs:513) and unsupported elem_size `_` arm (neon.rs:525). Documented: parse_reg_num returns None for invalid names; purpose comment lists b/h/s/d only.
- Seed: neon.rs encode_neon_dup_pbt encode_neon_dup_neg_arity_invalid
- Formal: ∀ rd ∈ [0,31], T ∈ valid, bad ∈ {foo,v32,v,empty,v-1,v99}, bad_sz ∈ {q,x,8b,16b,empty,w,4s}. encode_neon_dup([RegArrangement(v{rd},T), RegLane(bad, elem_of(T), i)]) = Err ∧ encode_neon_dup([RegArrangement(v{rd},T), RegLane(v0, bad_sz, 0)]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_dup
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, bad, bad_sz]
  domain: { rd: 0..31, t: valid T, bad: invalid names, bad_sz: unsupported elem_size }
  relation:
    op: throws
    expr: encode_neon_dup([RegArrangement(v{rd}, t), RegLane(bad_or_bad_sz)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: neon.rs:513 parse_reg_num None on element Rn; neon.rs:525 unsupported dup element size; parse_reg_num mod.rs:131-147
```
