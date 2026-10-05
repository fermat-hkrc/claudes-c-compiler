# Properties: encode_neon_elem_long

## encode_neon_elem_long_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the GNU-style assembly this encoder claims to accept. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree by-element-long decoder). Sibling encode_neon_elem / encode_neon_three_diff / encode_neon_float_elem rejected (same-job gate: non-widening by-element, vector three-diff, or FP by-element).
- Doc contract: neon.rs:230 "Encode NEON vector-by-element long instructions: SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL (elem)" — asserted fingerprint ea90509d
- Seed: encode_neon_cmp_zero_pbt.rs:166 llvm-mc differential; encode_neon_three_diff_pbt.rs widen/long table
- Formal: ∀ rd,rn ∈ {0..31}, (tb,ta,elem,imax,rmmax) ∈ {(4h,4s,h,7,15),(8h,4s,h,7,15),(2s,2d,s,3,31),(4s,2d,s,3,31)}, idx ∈ {0..imax}, rm ∈ {0..rmmax}, (U,opc,hi,mnem) ∈ SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL/SQDMULL/SQDMLAL/SQDMLSL (+2). hi ⇔ tb ∈ {8h,4s}. encode_neon_elem_long([Vd.ta, Vn.tb, Vm.elem[idx]], U, opc, hi) = llvm-mc(mnem Vd.ta, Vn.tb, Vm.elem[idx])
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, ta, elem, idx, u, opcode, is_high, mnem]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v15_or_v31, tb: {4h,8h,2s,4s} }
  relation:
    op: eq
    lhs: "encode_neon_elem_long(&[arr(rd,ta), arr(rn,tb), lane(rm,elem,idx)], u, opcode, is_high)"
    rhs: "llvm_mc_word(&format!(\"{mnem} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{elem}[{idx}]\"))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
evidence: README.md:12 README.md:230-231 neon.rs:230 encoder/mod.rs:318-329 encoder/mod.rs:749-890
```

## encode_neon_elem_long_meta_rd_rn_u
- Tier: 4
- Rationale: ARM vector-by-element-long layout isolates Rd at [4:0], Rn at [9:5], U at bit 29. Metamorphic: changing only one of those inputs must flip only that field. Weaker than differential; kept as an independent algebraic check that does not depend on llvm-mc.
- Doc contract: neon.rs:232 "Format: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint 6cd6cc4b
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_meta_rd_rn_u
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, tb ∈ {4h,8h,2s,4s}, idx in-range(tb), rm in-range(tb), opc ∈ {0b0010,0b0011,0b0110,0b0111,0b1010,0b1011}, hi ∈ {0,1}. let w(rd,rn,U)=encode_neon_elem_long([Vd.ta,Vn.tb,Vm.elem[idx]],U,opc,hi). (w(rd1,rn1,U) ⊕ w(rd2,rn1,U)) ∧ ¬0x1F = 0 ∧ w[4:0]=rd. (w(rd1,rn1,U) ⊕ w(rd1,rn2,U)) ∧ ¬(0x1F≪5) = 0 ∧ w[9:5]=rn. w(...,0) ⊕ w(...,1) = 1≪29
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm, tb, idx, opcode, is_high]
  domain: { rd1: v0_v31, rn1: v0_v31, tb: {4h,8h,2s,4s} }
  relation:
    op: eq
    lhs: "(w(rd1,rn1,0) ^ w(rd2,rn1,0)) & !0x1F"
    rhs: "0"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:232 ARM ARM Advanced SIMD vector x indexed element
```

## encode_neon_elem_long_inv_layout
- Tier: 4
- Rationale: Documented encoding format 0 Q U 01111 size L M Rm opcode H 0 Rn Rd is an exact structural invariant over the valid domain. Weaker than differential; independent of llvm-mc.
- Doc contract: neon.rs:232 "Format: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd" — asserted fingerprint 6cd6cc4b
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, tb ∈ {4h,8h,2s,4s}, idx in-range(tb), rm in-range(tb), U ∈ {0,1}, opc ∈ long-opcodes, hi ∈ {false,true}. let w = encode_neon_elem_long(...). w[31]=0 ∧ w[30]=Q(hi,tb) ∧ w[29]=U ∧ w[28:24]=0b01111 ∧ w[23:22]=size(tb) ∧ (H,L,M)=index_enc(tb,idx,rm) ∧ w[19:16]=Rm_lo ∧ w[15:12]=opc ∧ w[10]=0 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, idx, u, opcode, is_high]
  domain: { rd: v0_v31, tb: {4h,8h,2s,4s} }
  relation:
    op: eq
    lhs: "(w >> 24) & 0x1F"
    rhs: "0b01111"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: neon.rs:232
```

## encode_neon_elem_long_neg_arity
- Tier: 3
- Rationale: neon.rs:237 documents a minimum of 3 operands. Negative/error contract: fewer than 3 operands must return Err. Does not declare a maximum (extra-operand is a separate property vs llvm-mc).
- Doc contract: neon.rs:237 "NEON elem-long requires 3 operands" — domain-restriction fingerprint b57ff802
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ops with |ops|=n. encode_neon_elem_long(ops, U, opc, hi) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, tb]
  domain: { n: {0,1,2} }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&ops_of_len(n), 0, 0b1010, false)"
    rhs: "Err"
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
evidence: neon.rs:237
```

## encode_neon_elem_long_neg_extra
- Tier: 3
- Rationale: README.md:12 claims GNU-style assembly; llvm-mc rejects a fourth operand on smull-by-element. The helper's "< 3" check does not declare a maximum, so extra operands are invalid under the assembler contract, not under a documented helper exclusion. Negative/error: encode_neon_elem_long must Err when |ops|>=4.
- Doc contract: neon.rs:237 "NEON elem-long requires 3 operands" — domain-restriction fingerprint b57ff802 (minimum only; extra-operand contract inferred from README.md:12 + llvm-mc)
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_neg_extra
- Formal: ∀ rd,rn,rm ∈ {0..31}, valid (ta,tb,elem,idx,hi), extra operand. llvm-mc rejects mnem Vd.ta, Vn.tb, Vm.elem[idx], extra ⇒ encode_neon_elem_long([Vd.ta,Vn.tb,Vm.elem[idx],extra],...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem_long([v0.4s, v0.4h, v0.h[0], v0.4s], u=0, opc=0b1010, is_high=false) = Ok(Word) — extra operand ignored; llvm-mc rejects `smull v0.4s, v0.4h, v0.h[0], v0.4s`
- Bug report: pbt-out/bug_reports/encode_neon_elem_long_extra_operand.md

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, idx, extra]
  domain: { rd: v0_v31 }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,ta), arr(rn,tb), lane(rm,elem,idx), extra], u, opc, hi)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 llvm-mc rejects fourth operand
```

## encode_neon_elem_long_neg_mismatch_ta
- Tier: 3
- Rationale: ARM long by-element requires dest arrangement to be the widened form of the source (4h/8h→4s, 2s/4s→2d) and the `2` mnemonic to use the upper source half. llvm-mc rejects mismatched Ta/Tb. Dest `_arr_d` is discarded in the helper; this property asserts the assembler contract, not the producing discard.
- Doc contract: neon.rs:234 "These are the widening multiply-by-element forms where the third operand is a register lane (e.g., v0.h[2])." — asserted fingerprint b1827a83
- Seed: encode_neon_shll_pbt.rs encode_neon_shll_neg_dest_tb
- Formal: ∀ rd,rn,rm, tb ∈ {4h,8h,2s,4s}, ta' ≠ mandated_ta(tb), idx in-range. llvm-mc rejects smull Vd.ta', Vn.tb, Vm.elem[idx] ⇒ encode_neon_elem_long([Vd.ta',Vn.tb,Vm.elem[idx]], 0, 0b1010, hi(tb)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem_long([v0.8b, v0.4h, v0.h[0]], u=0, opc=0b1010, is_high=false) = Ok(Word) — dest arrangement discarded; llvm-mc rejects `smull v0.8b, v0.4h, v0.h[0]`
- Bug report: pbt-out/bug_reports/encode_neon_elem_long_mismatch_ta.md

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, ta_wrong, idx]
  domain: { tb: {4h,8h,2s,4s}, ta_wrong: {8b,16b,4h,8h,2s,4s,2d,1d} \\ mandated }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,ta_wrong), arr(rn,tb), lane(rm,elem,idx)], 0, 0b1010, hi)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["4h", "8h", "2s", "4s"] }
evidence: README.md:12 ARM ARM long by-element dest arrangement
```

## encode_neon_elem_long_neg_h_rm_hi
- Tier: 3
- Rationale: ARM size=01 (H) encodes Rm in 4 bits; llvm-mc rejects Vm in v16-v31 for .h lanes. The helper comment "Limit Rm for half-word indexing (only v0-v15)" is the producing mask `rm & 0xF`, not an API exclusion — the function accepts any parse_reg_num register. Contract: v16-v31 as .h lane must Err (inferred from ARM + llvm-mc + README GNU-style).
- Doc contract: neon.rs:289 "Limit Rm for half-word indexing (only v0-v15)" — limitation fingerprint (producing mask; not an input-domain restriction). neon.rs:234 asserted fingerprint b1827a83
- Seed: encode_neon_dup_pbt.rs lane index domain
- Formal: ∀ rd,rn ∈ {0..31}, rm ∈ {16..31}, idx ∈ {0..7}, tb ∈ {4h,8h}. llvm-mc rejects smull Vd.4s, Vn.tb, Vm.h[idx] ⇒ encode_neon_elem_long(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem_long([v0.4s, v0.4h, v16.h[0]], u=0, opc=0b1010, is_high=false) = Ok(Word) encoding v0.h[0] (rm & 0xF); llvm-mc rejects `smull v0.4s, v0.4h, v16.h[0]`
- Bug report: pbt-out/bug_reports/encode_neon_elem_long_h_rm_hi.md

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, idx]
  domain: { rm: {16..31}, tb: {4h,8h}, idx: {0..7} }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,\"4s\"), arr(rn,tb), lane(rm,\"h\",idx)], 0, 0b1010, hi)"
    rhs: "Err"
expected_error: String
generators:
  rm: { gen: int, min: 16, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: u32 }
evidence: ARM ARM size=01 Rm v0-v15; llvm-mc rejects v16.h[0]
```

## encode_neon_elem_long_neg_gpr_bare_nonv
- Tier: 3
- Rationale: GNU-style NEON long by-element requires Vd.Ta / Vn.Tb / Vm.elem[index]. llvm-mc rejects GPR dest, bare v-reg without arrangement, x/w prefix-as-arrangement, and a non-lane third operand. get_neon_reg accepts Operand::Reg and non-v prefixes; the third operand must be RegLane. Negative/error vs llvm-mc.
- Doc contract: neon.rs:234 "These are the widening multiply-by-element forms where the third operand is a register lane (e.g., v0.h[2])." — asserted fingerprint b1827a83
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_neg_gpr_bare_nonv
- Formal: ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, x-arranged, non-lane-third, s-dest}. llvm-mc rejects the corresponding asm ⇒ encode_neon_elem_long(ops, 0, 0b1010, false) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem_long([x0, v0.4h, v0.h[0]], u=0, opc=0b1010, is_high=false) = Ok(Word); llvm-mc rejects `smull x0, v0.4h, v0.h[0]`
- Bug report: pbt-out/bug_reports/encode_neon_elem_long_gpr_dest.md

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { kind: {x_dest, w_dest, sp_dest, bare_v, x_arranged, non_lane, s_dest} }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&ops(kind), 0, 0b1010, false)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u8 }
evidence: README.md:12 neon.rs:234 llvm-mc rejects non-V / non-lane forms
```

## encode_neon_elem_long_neg_index_oob
- Tier: 3
- Rationale: neon.rs:266/276 document index-out-of-range Err for .h (index>7) and .s (index>3). Sweep: documented error path not in the first batch.
- Doc contract: neon.rs:266 "element index {} out of range for .h" — domain-restriction fingerprint f6aad7d7
- Seed: encode_neon_dup_pbt.rs encode_neon_dup_neg_index_oor
- Formal: ∀ valid shape with imax, idx > imax. llvm-mc rejects mnem Vd.ta, Vn.tb, Vm.elem[idx] ⇒ encode_neon_elem_long(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, idx]
  domain: { idx: imax+1 .. imax+8 }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,ta), arr(rn,tb), lane(rm,elem,idx)], 0, 0b1010, hi)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 8, max: 15, type: u32 }
evidence: neon.rs:266 neon.rs:276
```

## encode_neon_elem_long_neg_unsupported_src
- Tier: 3
- Rationale: neon.rs:258 documents Err for source arrangements other than 4h/8h/2s/4s. Sweep: documented error path.
- Doc contract: neon.rs:258 "unsupported source arrangement for elem-long" — domain-restriction fingerprint 1fdf6624
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_neg_invalid_t
- Formal: ∀ tb ∉ {4h,8h,2s,4s}. encode_neon_elem_long([Vd.4s, Vn.tb, Vm.h[idx]], ...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb]
  domain: { tb: {8b,16b,2d,1d,4b,""} }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,\"4s\"), arr(rn,tb), lane(rm,\"h\",idx)], 0, 0b1010, false)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "2d", "1d", "4b", ""] }
evidence: neon.rs:258
```

## encode_neon_elem_long_neg_lane_elem_mismatch
- Tier: 3
- Rationale: neon.rs:234 states the third operand is a register lane such as v0.h[2]; llvm-mc requires the lane elem size to match the source element size. The helper binds `elem_size: _` and ignores it. Sweep: documented lane form.
- Doc contract: neon.rs:234 "These are the widening multiply-by-element forms where the third operand is a register lane (e.g., v0.h[2])." — asserted fingerprint b1827a83
- Seed: encode_neon_dup_pbt.rs lane elem domain
- Formal: ∀ valid (ta,tb,elem,idx), wrong ≠ elem. llvm-mc rejects mnem Vd.ta, Vn.tb, Vm.wrong[idx] ⇒ encode_neon_elem_long(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: failing
- Counterexample: encode_neon_elem_long([v0.2d, v0.2s, v0.b[0]], u=0, opc=0b1010, is_high=false) = Ok(Word); llvm-mc rejects `smull v0.2d, v0.2s, v0.b[0]`
- Bug report: pbt-out/bug_reports/encode_neon_elem_long_lane_elem_mismatch.md

```property
function: encoder.encode_neon_elem_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, wrong, idx]
  domain: { wrong: {b,h,s,d} \\ elem }
  relation:
    op: throws
    lhs: "encode_neon_elem_long(&[arr(rd,ta), arr(rn,tb), lane(rm,wrong,idx)], 0, 0b1010, hi)"
    rhs: "Err"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  wrong: { gen: oneof, values: ["b", "h", "s", "d"] }
evidence: neon.rs:234 README.md:12 llvm-mc requires matching lane elem size
```

## encode_neon_elem_long_diff_alt_spellings
- Tier: 5
- Rationale: parse_reg_num lowercases; GNU assemblers accept uppercase V. Sweep differential vs llvm-mc on uppercase-V spellings of the valid domain.
- Doc contract: neon.rs:230 "Encode NEON vector-by-element long instructions: SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL (elem)" — asserted fingerprint ea90509d
- Seed: encode_neon_cmp_zero_pbt.rs encode_neon_cmp_zero_diff_alt_spellings
- Formal: ∀ valid-domain inputs. encode_neon_elem_long([Vrd.ta, Vrn.tb, Vrm.elem[idx]], ...) = llvm-mc(mnem Vrd.ta, Vrn.tb, Vrm.elem[idx])
- Test file: src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_elem_long
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, idx, u, opcode, is_high, mnem]
  domain: { rd: v0_v31 }
  relation:
    op: eq
    lhs: "encode_neon_elem_long(&[Arr(Vrd,ta), Arr(Vrn,tb), Lane(Vrm,elem,idx)], u, opcode, is_high)"
    rhs: "llvm_mc_word(uppercase-V asm)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 neon.rs:230 parse_reg_num lowercases
```
