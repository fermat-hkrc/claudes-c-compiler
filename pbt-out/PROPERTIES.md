# Properties: encode_neon_ins

## encode_neon_ins_diff_llvm_mc_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, which the assembler README claims gas-compatible encodings. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree INS decoder). Sibling encode_neon_dup / encode_neon_umov rejected by same-job gate (DUP 000011, UMOV 001111 vs INS general 000111).
- Doc contract: neon.rs:548 "Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn" — asserted fingerprint e12aeb4a
- Seed: neon.rs:11214 encode_neon_dup_pbt llvm-mc differential
- Formal: ∀ rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. encode_neon_ins([Vd.ts[i], R(ts,rn)]) = llvm-mc("ins Vd.ts[i], R") where R is Wn for ts∈{b,h,s} and Xn for ts=d (wzr/xzr at 31)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: v0_31, rn: gpr0_31, ts: bhsd, i: 0_imax }
  relation:
    op: eq
    lhs: encode_neon_ins(gpr_ops(rd, ts, i, rn))
    rhs: llvm_mc_word(ins_gpr_asm(rd, ts, i, rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ins_diff_llvm_mc_elem
- Tier: 5
- Rationale: Element-to-element INS is the second ARM form listed in README.md:234 `ins` (element/GPR) and neon.rs:598. Same differential reference as GPR form.
- Doc contract: neon.rs:598 "INS Vd.Ts[dst], Vn.Ts[src]: 0 1 1 01110 000 imm5 0 imm4 1 Rn Rd" — asserted fingerprint 64c1b9b2
- Seed: neon.rs:11214 encode_neon_dup_pbt element form
- Formal: ∀ rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, di,si ∈ [0, imax(ts)]. encode_neon_ins([Vd.ts[di], Vn.ts[si]]) = llvm-mc("ins Vd.ts[di], Vn.ts[si]")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ts, di, si]
  domain: { rd: v0_31, rn: v0_31, ts: bhsd, di: 0_imax, si: 0_imax }
  relation:
    op: eq
    lhs: encode_neon_ins(elem_ops(rd, ts, di, rn, ts, si))
    rhs: llvm_mc_word(ins_elem_asm(rd, ts, di, rn, ts, si))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  di: { gen: int, min: 0, max: 15, type: u32 }
  si: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:234
```

## encode_neon_ins_metamorphic_rd_rn
- Tier: 4
- Rationale: ARM INS Rd occupies bits[4:0] and Rn bits[9:5]. Changing only Rd (resp. Rn) must differ only in that field. Stronger differential already used as primary; this metamorphic pins field placement independently of llvm-mc.
- Doc contract: neon.rs:567 "INS Vd.Ts[i], Xn: 0 1 0 01110 000 imm5 0 0011 1 Rn Rd" — asserted fingerprint 9250ce99
- Seed: encode_neon_tbx_metamorphic_q
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. let w(rd,rn)=encode_neon_ins(gpr_ops(rd,ts,i,rn)). (w(rd1,rn1) XOR w(rd2,rn1)) & ~0x1F = 0 ∧ (w(rd1,rn1) XOR w(rd1,rn2)) & ~(0x1F<<5) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, ts, i]
  domain: { rd1: v0_31, rd2: v0_31, rn1: gpr0_31, rn2: gpr0_31, ts: bhsd, i: 0_imax }
  body: ((w(rd1,rn1) XOR w(rd2,rn1)) & ~0x1F == 0) && ((w(rd1,rn1) XOR w(rd1,rn2)) & ~(0x1F<<5) == 0)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:567
```

## encode_neon_ins_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD copy INS field layout; pins general vs element opcode bit 29 and imm5/imm4 packing.
- Doc contract: neon.rs:567 "INS Vd.Ts[i], Xn: 0 1 0 01110 000 imm5 0 0011 1 Rn Rd" — asserted fingerprint 9250ce99
- Seed: encode_neon_tbx_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. let w = encode_neon_ins(gpr_ops). w[31]=0 ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=01110000 ∧ w[20:16]=imm5(ts,i) ∧ w[15:10]=000111 ∧ w[9:5]=rn ∧ w[4:0]=rd. And for element form w[29]=1 ∧ w[15]=0 ∧ w[14:11]=imm4(ts,si) ∧ w[10]=1.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i, si]
  domain: { rd: v0_31, rn: v0_31, ts: bhsd, i: 0_imax, si: 0_imax }
  body: arm_ins_general_fields(w_gpr) && arm_ins_elem_fields(w_elem)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
  si: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:567
```

## encode_neon_ins_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a third operand; README gas-compat requires Err. SUT currently checks operands.len() < 2 so extras are ignored.
- Doc contract: neon.rs:551 "ins requires 2 operands" — asserted fingerprint bf810b20
- Seed: encode_neon_dup_extra_operand / encode_neon_tbx_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. llvm-mc("ins Vd.ts[i], R, extra") fails ⇒ encode_neon_ins(gpr_ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: failing
- Counterexample: encode_neon_ins([v0.b[0], w0, w0]) = Ok(Word(0x4e011c00))
- Bug report: bug_reports/encode_neon_ins_extra_operand.md

```property
function: encoder.encode_neon_ins
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ts, i]
  domain: { rd: v0_31, rn: gpr0_31, extra: v0_31, ts: bhsd, i: 0_imax }
  body: encode_neon_ins(gpr_ops_plus_extra).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ins_neg_index_oor
- Tier: 3
- Rationale: ARM / llvm-mc require lane in [0, imax(ts)]; bound and bound+1 must be exercised. SUT masks index bits instead of rejecting.
- Doc contract: neon.rs:548 "Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn" — asserted fingerprint e12aeb4a
- Seed: encode_neon_dup_index_oor
- Formal: ∀ rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ {imax(ts)+1 .. imax(ts)+8}. encode_neon_ins([Vd.ts[i], R]) = Err ∧ encode_neon_ins([Vd.ts[0], Vn.ts[i]]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: failing
- Counterexample: encode_neon_ins([v0.b[16], w0]) = Ok(Word(0x4e011c00)) (lane 16 masked to 0)
- Bug report: bug_reports/encode_neon_ins_index_oor.md

```property
function: encoder.encode_neon_ins
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: v0_31, rn: gpr0_31, ts: bhsd, i: imax_plus }
  body: encode_neon_ins(gpr_ops(rd, ts, i, rn)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 1, max: 16, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ins_neg_wrong_width_gpr
- Tier: 3
- Rationale: ARM INS (general) takes Wn for Ts in {B,H,S} and Xn for Ts=D. llvm-mc rejects the swapped width. SUT never inspects the GPR prefix.
- Doc contract: neon.rs:548 "Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn" — asserted fingerprint e12aeb4a
- Seed: encode_neon_dup_wrong_width_gpr
- Formal: ∀ rd,rn ∈ {0..30}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. encode_neon_ins([Vd.ts[i], wrong_width(ts,rn)]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: failing
- Counterexample: encode_neon_ins([v0.b[0], x0]) = Ok(Word(0x4e011c00))
- Bug report: bug_reports/encode_neon_ins_wrong_width_gpr.md

```property
function: encoder.encode_neon_ins
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: v0_31, rn: 0_30, ts: bhsd, i: 0_imax }
  body: encode_neon_ins(wrong_width_ops(rd, ts, i, rn)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ins_neg_mismatch_sp_fp
- Tier: 3
- Rationale: llvm-mc rejects mismatched element sizes, SP/WSP as GPR source, and FP/SIMD names as the general-form source. README gas-compat requires Err. SUT ignores _src_size, maps SP to 31, and parse_reg_num accepts d/s/q/v/h/b prefixes.
- Doc contract: neon.rs:603 "ins: expected (RegLane, Reg) or (RegLane, RegLane) operands" — asserted fingerprint 526a12ca
- Seed: encode_neon_dup_size_mismatch / encode_neon_dup_wrong_width_gpr
- Formal: ∀ rd,rn ∈ {0..31}, ts ≠ ts2 ∈ {b,h,s,d}, i ∈ [0, imax(ts)], j ∈ [0, imax(ts2)]. encode_neon_ins([Vd.ts[i], Vn.ts2[j]]) = Err ∧ encode_neon_ins([Vd.ts[i], SP|WSP|dN|sN|qN|vN]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: failing
- Counterexample: encode_neon_ins([v0.b[0], sp]) = Ok(Word(0x4e011fe0)) (SP encoded as XZR/WZR)
- Bug report: bug_reports/encode_neon_ins_sp_as_zr.md

```property
function: encoder.encode_neon_ins
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, ts2, i, j, kind]
  domain: { rd: v0_31, rn: 0_31, ts: bhsd, ts2: bhsd, i: 0_imax, j: 0_imax, kind: 0_3 }
  body: encode_neon_ins(invalid_ops(kind, rd, rn, ts, ts2, i, j)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  ts2: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
  j: { gen: int, min: 0, max: 15, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u8 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ins_neg_arity
- Tier: 3
- Rationale: Documented error path neon.rs:551 "ins requires 2 operands" for arity < 2. Invalid dest names (v32/foo/empty) must Err. Sweep property for the len < 2 branch.
- Doc contract: neon.rs:551 "ins requires 2 operands" — asserted fingerprint bf810b20
- Seed: encode_neon_tbx_neg_arity_kinds
- Formal: ∀ n ∈ {0,1}, rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. encode_neon_ins(ops[..n]) = Err. ∀ bad ∈ {v32, foo, empty, v}. encode_neon_ins([RegLane(bad,ts,i), R]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, ts, i, bad]
  domain: { n: 0_1, rd: v0_31, rn: gpr0_31, ts: bhsd, i: 0_imax, bad: names }
  body: encode_neon_ins(ops.take(n)).is_err() && encode_neon_ins(bad_dest).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:551
```

## encode_neon_ins_diff_alt_spellings
- Tier: 5
- Rationale: README gas-compat plus llvm-mc accept uppercase V/W/X register spellings; parse_reg_num lowercases. Sweep differential for caller-reachable case variants.
- Doc contract: neon.rs:548 "Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn" — asserted fingerprint e12aeb4a
- Seed: encode_neon_tbx_diff_alt_spellings
- Formal: ∀ rd ∈ {0..31}, rn ∈ {0..30}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. encode_neon_ins([V{rd}.ts[i], W|X{rn}]) = llvm-mc("ins V{rd}.ts[i], W|X{rn}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ins
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: v0_31, rn: 0_30, ts: bhsd, i: 0_imax }
  relation:
    op: eq
    lhs: encode_neon_ins(alt_ops(rd, ts, i, rn))
    rhs: llvm_mc_word(alt_asm(rd, ts, i, rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  ts: { gen: oneof, items: ["b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```
