# Properties: encode_neon_elem

## encode_neon_elem_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc. README.md:12 claims GNU-style assembly; README.md:231 lists mul/mla/mls/sqdmulh/sqrdmulh with lane index; neon.rs:1590 asserts the ARM vector-x-indexed-element layout. State machine rejected (pure function). Round-trip rejected (no in-tree by-element decoder). Sibling encode_neon_elem_long / encode_neon_mul / encode_neon_mla / encode_neon_mls / encode_neon_float_elem rejected (same-job gate: widening / vector / FP).
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:250 llvm-mc differential
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {4h,8h,2s,4s}, idx ∈ [0, imax(T)], (U,opc,mnem) ∈ ARM_ELEM_TABLE. (T=4h∨T=8h ⇒ rm≤15) ⇒ encode_neon_elem([Vd.T,Vn.T,Vm.Ts[idx]], U, opc) = llvm-mc(mnem Vd.T, Vn.T, Vm.Ts[idx])
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t, insn]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: v0_v31_or_v0_v15
    idx: 0_imax_T
    t: fourh_eighth_twos_fours
    insn: ARM_ELEM_TABLE
  relation:
    op: eq
    lhs: encode_neon_elem(ops, insn.u, insn.opcode)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
evidence: README.md:12 README.md:231 neon.rs:1590 encoder/mod.rs:307-310
```

## encode_neon_elem_meta_rd_rn_u
- Tier: 4
- Rationale: Algebraic metamorphic isolation of Rd/Rn/U fields from ARM layout neon.rs:1590. Stronger Differential is the sibling success-path property; this checks field independence without llvm-mc. Round-trip rejected (no decoder).
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:271 Rd/Rn/U isolation
- Formal: ∀ rd1,rd2,rn1,rn2,rm,idx,T,opc. valid(T,rm,idx) ⇒ (encode(rd1,rn1,U=0) ⊕ encode(rd2,rn1,U=0)) & ~0x1F = 0 ∧ (encode(rd1,rn1,U=0) ⊕ encode(rd1,rn2,U=0)) & ~(0x1F<<5) = 0 ∧ encode(rd1,rn1,U=0) ⊕ encode(rd1,rn1,U=1) = 1<<29
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm, idx, t, opcode]
  domain:
    rd1: v0_v31
    rd2: v0_v31
    rn1: v0_v31
    rn2: v0_v31
    rm: constrained
    idx: constrained
    t: fourh_eighth_twos_fours
    opcode: ARM_ELEM_OPCODES
  body: (w11 ^ w21) & !0x1F == 0 && (w11 ^ w12) & !(0x1F << 5) == 0 && (w11 ^ w_u1) == (1 << 29)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
  opcode: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:1590
```

## encode_neon_elem_inv_layout
- Tier: 4
- Rationale: Algebraic invariant of the ARM vector-x-indexed-element word. neon.rs:1590 documents 0 Q U 01111 size L M Rm opcode H 0 Rn Rd. Stronger Differential is the sibling success-path property.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:307 layout invariant
- Formal: ∀ rd,rn,rm,idx,T,U,opc. valid(T,rm,idx) ⇒ let w = encode(...) in w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=U ∧ w[28:24]=01111 ∧ w[23:22]=size(T) ∧ w[21]=L ∧ w[20]=M ∧ w[19:16]=Rm[3:0] ∧ w[15:12]=opc ∧ w[11]=H ∧ w[10]=0 ∧ w[9:5]=Rn ∧ w[4:0]=Rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t, u, opcode]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: constrained
    t: fourh_eighth_twos_fours
    u: 0_1
    opcode: ARM_ELEM_OPCODES
  body: layout_holds(w, rd, rn, rm, idx, t, u, opcode)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:1590
```

## encode_neon_elem_neg_arity
- Tier: 3
- Rationale: Negative/error contract. neon.rs:1592 returns Err("NEON by-element requires 3 operands") when operands.len() < 3. Documented minimum arity; does not declare a maximum (extra is a separate property).
- Doc contract: neon.rs:1592 "NEON by-element requires 3 operands" — domain-restriction fingerprint 4b60f4a7
- Seed: encode_neon_elem_long_pbt.rs:341 arity
- Formal: ∀ n ∈ {0,1,2}, ops with |ops|=n. encode_neon_elem(ops, U, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, t]
  domain:
    n: 0_2
    rd: v0_v31
    rn: v0_v31
    t: fourh_eighth_twos_fours
  relation:
    op: holds
    expr: encode_neon_elem(ops_of_len(n), 0, 8).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1592
```

## encode_neon_elem_neg_extra
- Tier: 3
- Rationale: Negative/error vs llvm-mc/gas. README.md:12 GNU-style assembly; llvm-mc rejects a fourth operand. neon.rs:1592 documents a minimum of 3 operands, not a maximum — extra is still invalid assembly. Contract inferred from the public assembler (wrapper encode() passes operands through at mod.rs:307-310 / 793-796).
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:361 extra operand
- Formal: ∀ valid by-element triple + extra operand. llvm-mc rejects ⇒ encode_neon_elem(ops++[extra], U, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([v0.4h, v0.4h, v0.h[0], v0.4h], u=0, opcode=0b1000) → Ok(Word); llvm-mc rejects `mul v0.4h, v0.4h, v0.h[0], v0.4h`
- Bug report: bug_reports/encode_neon_elem_extra_operand.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, extra, t, insn]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: constrained
    extra: v0_v31
    t: fourh_eighth_twos_fours
    insn: ARM_ELEM_TABLE
  relation:
    op: holds
    expr: encode_neon_elem(ops_plus_extra, insn.u, insn.opcode).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc rejects fourth operand encoder/mod.rs:307-310
```

## encode_neon_elem_neg_mismatch_t
- Tier: 3
- Rationale: Negative/error vs llvm-mc/gas. ARM requires Vd.T and Vn.T to match; llvm-mc rejects mismatched source arrangement. SUT-boundary: encode() passes operands through; wrapper does not sanitize arrangements.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:395 mismatch Ta
- Formal: ∀ T, T' ≠ T ∈ {8b,16b,4h,8h,2s,4s,2d,1d}. llvm-mc rejects mul Vd.T, Vn.T', Vm.Ts[idx] ⇒ encode_neon_elem([Vd.T, Vn.T', Vm.Ts[idx]], 0, 0b1000) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([v0.4h, v0.8b, v0.h[0]], u=0, opcode=0b1000) → Ok(Word); llvm-mc rejects `mul v0.4h, v0.8b, v0.h[0]`
- Bug report: bug_reports/encode_neon_elem_mismatch_t.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t, t_wrong]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: constrained
    t: fourh_eighth_twos_fours
    t_wrong: neon_t_not_equal_t
  relation:
    op: holds
    expr: encode_neon_elem(mismatched_ops, 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: README.md:12 ARM matching T llvm-mc encoder/mod.rs:307-310
```

## encode_neon_elem_neg_h_rm_hi
- Tier: 3
- Rationale: Negative/error vs ARM/llvm-mc. size=01 (H) restricts Rm to v0-v15; llvm-mc rejects v16.h[idx]. The SUT mask `rm & 0xF` is the producing statement, not an API exclusion of v16-v31.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:424 H-lane Rm v16-v31
- Formal: ∀ rd,rn ∈ {0..31}, rm ∈ {16..31}, idx ∈ {0..7}, T ∈ {4h,8h}. llvm-mc rejects mul Vd.T, Vn.T, Vm.h[idx] ⇒ encode_neon_elem(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([v0.4h, v0.4h, v16.h[0]], u=0, opcode=0b1000) → Ok(Word) encoding v0.h[0]; llvm-mc rejects `mul v0.4h, v0.4h, v16.h[0]`
- Bug report: bug_reports/encode_neon_elem_h_rm_hi.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: 16_31
    idx: 0_7
    t: fourh_eighth
  relation:
    op: holds
    expr: encode_neon_elem(h_rm_hi_ops, 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 16, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: ARM size=01 Rm v0-v15 llvm-mc README.md:12
```

## encode_neon_elem_neg_gpr_bare_nonv
- Tier: 3
- Rationale: Negative/error vs llvm-mc/gas. GNU-style by-element requires arranged V registers and a lane third operand. get_neon_reg accepting Operand::Reg and parse_reg_num accepting x/w/s prefixes is not an API contract that GPR dest is valid.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:448 GPR/bare/non-V
- Formal: ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-V, x-prefix arrangement, non-lane third, s-dest}. llvm-mc rejects ⇒ encode_neon_elem(ops, 0, 0b1000) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([x0.4h, v0.4h, v0.h[0]], u=0, opcode=0b1000) → Ok(Word) as V0; llvm-mc rejects `mul x0.4h, v0.4h, v0.h[0]`
- Bug report: bug_reports/encode_neon_elem_x_prefix.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, kind]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: 0_15
    idx: 0_7
    kind: 0_6
  relation:
    op: holds
    expr: encode_neon_elem(non_neon_ops(kind), 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 15, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: README.md:12 llvm-mc encoder/mod.rs:307-310
```

## encode_neon_elem_neg_index_oob
- Tier: 3
- Rationale: Sweep — ARM/llvm-mc reject lane index > 7 for .h and > 3 for .s. encode_neon_elem_long range-checks; encode_neon_elem does not. Documented bound from ARM vector-x-indexed-element (H:L:M / H:L) and llvm-mc error "vector lane must be an integer in range [0, 7]".
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:512 index-oob
- Formal: ∀ T ∈ {4h,8h,2s,4s}, idx > imax(T). llvm-mc rejects mul Vd.T, Vn.T, Vm.Ts[idx] ⇒ encode_neon_elem(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([v0.4h, v0.4h, v0.h[8]], u=0, opcode=0b1000) → Ok(Word) wrapping to index 0; llvm-mc rejects `mul v0.4h, v0.4h, v0.h[8]`
- Bug report: bug_reports/encode_neon_elem_index_oob.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: imax_plus
    t: fourh_eighth_twos_fours
  relation:
    op: holds
    expr: encode_neon_elem(index_oob_ops, 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM index H:L:M 0..7 / H:L 0..3 llvm-mc README.md:12
```

## encode_neon_elem_neg_unsupported_t
- Tier: 3
- Rationale: Sweep — neon.rs:1603 declares size not in {01,10} invalid ("unsupported element size for by-element"). Dest T whose neon_arr_to_q_size size is 00 or 11 (8b/16b/2d/1d) or unknown (4b, empty) is out of domain; the property asserts the documented Err, not a mishandling of valid input.
- Doc contract: neon.rs:1603 "unsupported element size for by-element" — domain-restriction fingerprint 630d7964
- Seed: encode_neon_elem_long_pbt.rs:541 unsupported-src
- Formal: ∀ T ∈ documented-invalid-size {8b,16b,2d,1d,4b,ε}. encode_neon_elem([Vd.T, Vn.T, Vm.h[idx]], 0, 0b1000) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: 0_15
    idx: 0_3
    t: unsupported_t
  relation:
    op: holds
    expr: encode_neon_elem(unsupported_ops, 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 15, type: u32 }
  idx: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: neon.rs:1603
```

## encode_neon_elem_neg_lane_elem_mismatch
- Tier: 3
- Rationale: Sweep — llvm-mc rejects Vm.b[idx] when T is .4h (lane elem must match arrangement). SUT discards elem_size (`RegLane { reg, index, .. }`). Not an API exclusion.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:557 lane-elem-mismatch
- Formal: ∀ T ∈ {4h,8h,2s,4s}, wrong ≠ Ts(T). llvm-mc rejects mul Vd.T, Vn.T, Vm.wrong[idx] ⇒ encode_neon_elem(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem([v0.4h, v0.4h, v0.b[0]], u=0, opcode=0b1000) → Ok(Word); llvm-mc rejects `mul v0.4h, v0.4h, v0.b[0]`
- Bug report: bug_reports/encode_neon_elem_lane_elem_mismatch.md

```property
function: encoder.encode_neon_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t, wrong]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: constrained
    t: fourh_eighth_twos_fours
    wrong: b_h_s_d_not_elem
  relation:
    op: holds
    expr: encode_neon_elem(lane_mismatch_ops, 0, 8).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc lane elem must match T
```

## encode_neon_elem_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — parse_reg_num lowercases V/X/W prefixes; llvm-mc accepts uppercase V. Differential agreement on uppercase V spellings of the valid domain.
- Doc contract: neon.rs:1590 "MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint c300f7ba
- Seed: encode_neon_elem_long_pbt.rs:587 alt-spellings
- Formal: ∀ valid by-element with uppercase V prefix. encode_neon_elem([Vrd.T, Vrn.T, Vrm.Ts[idx]], U, opc) = llvm-mc(mnem Vrd.T, Vrn.T, Vrm.Ts[idx])
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t, insn]
  domain:
    rd: v0_v31
    rn: v0_v31
    rm: constrained
    idx: constrained
    t: fourh_eighth_twos_fours
    insn: ARM_ELEM_TABLE
  relation:
    op: eq
    lhs: encode_neon_elem(uppercase_ops, insn.u, insn.opcode)
    rhs: llvm_mc(uppercase_asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
evidence: README.md:12 parse_reg_num lowercases prefix
```
