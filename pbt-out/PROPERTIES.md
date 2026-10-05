# Properties: encode_neon_umov

## encode_neon_umov_diff_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, the independent AArch64 assembler the README claims gas-compatible encodings against. State machine rejected (pure function). Round-trip rejected (no in-tree UMOV decoder). Sibling encode_neon_dup / encode_neon_ins / encode_mov rejected (same-job gate: different opcodes / MOV is a multi-form alias encoder).
- Doc contract: neon.rs:460 "Encode NEON UMOV: move element to GP register" — asserted fingerprint 484130f7
- Seed: encode_neon_ins_pbt.rs:277 encode_neon_ins_diff_llvm_mc_gpr
- Formal: ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. encode_neon_umov([Reg(gpr(ts,rd)), RegLane(v{rn}, ts, i)]) = llvm-mc("umov {W|X}{rd}, v{rn}.{ts}[{i}]")
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: "0..=31", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  relation:
    op: eq
    lhs: encode_neon_umov([Reg(gpr(ts,rd)), RegLane(v{rn}, ts, i)])
    rhs: llvm_mc("umov {W|X}{rd}, v{rn}.{ts}[{i}]")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_metamorphic_rd_rn
- Tier: 4
- Rationale: Algebraic metamorphic on Rd/Rn fields; weaker than differential but independent of llvm-mc. ARM encoding places Rd at bits[4:0] and Rn at bits[9:5].
- Doc contract: neon.rs:482 "UMOV Rd, Vn.Ts[index]: 0 Q 0 01110 000 imm5 0 0111 1 Rn Rd" — asserted fingerprint 730a17d5
- Seed: encode_neon_ins_pbt.rs:307 encode_neon_ins_metamorphic_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. let w11 = umov(rd1,rn1,ts,i), w21 = umov(rd2,rn1,ts,i), w12 = umov(rd1,rn2,ts,i). (w11 ⊕ w21) ∧ ¬0x1F = 0 ∧ (w21 ∧ 0x1F) = rd2 ∧ (w11 ⊕ w12) ∧ ¬(0x1F≪5) = 0 ∧ ((w12≫5) ∧ 0x1F) = rn2
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, ts, i]
  domain: { rd1: "0..=31", rd2: "0..=31", rn1: "0..=31", rn2: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  body: "(w11 xor w21) & !0x1F == 0 && (w21 & 0x1F) == rd2 && (w11 xor w12) & !(0x1F << 5) == 0 && ((w12 >> 5) & 0x1F) == rn2"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:482
```

## encode_neon_umov_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant on the documented ARM UMOV layout. Q=1 iff Ts=D; bits[28:21]=01110000; imm5 encodes size+index; bits[15:10]=001111.
- Doc contract: neon.rs:482 "UMOV Rd, Vn.Ts[index]: 0 Q 0 01110 000 imm5 0 0111 1 Rn Rd" — asserted fingerprint 730a17d5
- Seed: encode_neon_ins_pbt.rs:335 encode_neon_ins_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. let w = encode_neon_umov(...). (w≫31)∧1=0 ∧ (w≫30)∧1 = [ts=d] ∧ (w≫29)∧1=0 ∧ (w≫21)∧0xFF=0b01110000 ∧ (w≫16)∧0x1F=imm5(ts,i) ∧ (w≫10)∧0x3F=0b001111 ∧ (w≫5)∧0x1F=rn ∧ w∧0x1F=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: "0..=31", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  body: "bit31=0 && Q=(ts==d) && bit29=0 && bits[28:21]=01110000 && imm5 && bits[15:10]=001111 && Rn && Rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:482
```

## encode_neon_umov_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a third operand; README gas-compatibility requires the encoder to Err. The SUT only checks operands.len() < 2.
- Doc contract: neon.rs:463 "umov requires 2 operands" — asserted fingerprint aa68d80e
- Seed: encode_neon_ins_pbt.rs:372 encode_neon_ins_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. llvm-mc("umov gpr, Vn.Ts[i], extra") = Err ⇒ encode_neon_umov([Reg, RegLane, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: failing
- Counterexample: encode_neon_umov([w0, v0.b[0], w0])
- Bug report: pbt-out/bug_reports/encode_neon_umov_extra_operand.md

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ts, i]
  domain: { rd: "0..=31", rn: "0..=31", extra: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  relation:
    op: holds
    expr: encode_neon_umov(ops_plus_extra).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_neg_index_oor
- Tier: 4
- Rationale: Negative/error contract. ARM/llvm-mc lane ranges are b[0-15] h[0-7] s[0-3] d[0-1]. The SUT masks the index (`index & 0xF` etc.) instead of rejecting OOR.
- Doc contract: neon.rs:467 "Second operand should be a RegLane (v0.b[0])" — asserted fingerprint e26be721
- Seed: encode_neon_ins_pbt.rs:396 encode_neon_ins_neg_index_oor
- Formal: ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ over ∈ {1..8}. let i = imax(ts)+over. llvm-mc("umov gpr, Vn.Ts[i]") = Err ⇒ encode_neon_umov(...) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: failing
- Counterexample: encode_neon_umov([w0, v0.b[16]])
- Bug report: pbt-out/bug_reports/encode_neon_umov_index_oor.md

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, over]
  domain: { rd: "0..=31", rn: "0..=31", ts: "{b,h,s,d}", over: "1..=8" }
  relation:
    op: holds
    expr: encode_neon_umov([Reg(gpr), RegLane(v{rn}, ts, imax(ts)+over)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  over: { gen: int, min: 1, max: 8, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_neg_wrong_width
- Tier: 4
- Rationale: Negative/error contract. ARM UMOV requires Wd for Ts in {B,H,S} and Xd for Ts=D. llvm-mc rejects the crossed pairing. The SUT sets Q from dest width (`is_64`) and encodes anyway.
- Doc contract: neon.rs:460 "Encode NEON UMOV: move element to GP register" — asserted fingerprint 484130f7
- Seed: encode_neon_ins_pbt.rs:431 encode_neon_ins_neg_wrong_width_gpr
- Formal: ∀ rd ∈ {0..30}, ∀ rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. llvm-mc("umov {wrong-width gpr}, Vn.Ts[i]") = Err ⇒ encode_neon_umov([Reg(wrong), RegLane]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: failing
- Counterexample: encode_neon_umov([x0, v0.b[0]])
- Bug report: pbt-out/bug_reports/encode_neon_umov_wrong_width.md

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: "0..=30", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  relation:
    op: holds
    expr: encode_neon_umov([Reg(wrong_gpr(ts,rd)), RegLane(v{rn}, ts, i)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_neg_arity_sp_fp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects arity 0/1, SP/WSP dest, and FP dest (d/s/q/v/h/b). parse_reg_num maps sp/wsp to 31 and accepts FP prefixes. Arity 0/1 and invalid names Err as required; SP/WSP/FP dest currently encode.
- Doc contract: neon.rs:463 "umov requires 2 operands" — asserted fingerprint aa68d80e
- Seed: encode_neon_ins_pbt.rs:457 encode_neon_ins_neg_mismatch_sp_fp / encode_neon_ins_neg_arity
- Formal: ∀ n ∈ {0,1}, ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)], ∀ kind ∈ {arity, sp, wsp, fp, bad-name}. llvm-mc rejects the corresponding assembly ⇒ encode_neon_umov returns Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: failing
- Counterexample: encode_neon_umov([sp, v0.b[0]])
- Bug report: pbt-out/bug_reports/encode_neon_umov_sp_fp_dest.md

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, ts, i, kind]
  domain: { n: "0..=1", rd: "0..=31", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]", kind: "{arity,sp,wsp,fp,bad-name}" }
  relation:
    op: holds
    expr: encode_neon_umov(invalid_ops).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_diff_alt_spellings
- Tier: 3
- Rationale: Differential vs llvm-mc on uppercase W/X/V register spellings. parse_reg_num lowercases; llvm-mc accepts them. Parser lowercases elem_size so Ts stays lowercase (parser.rs:1945).
- Doc contract: neon.rs:460 "Encode NEON UMOV: move element to GP register" — asserted fingerprint 484130f7
- Seed: encode_neon_ins_pbt.rs:542 encode_neon_ins_diff_alt_spellings
- Formal: ∀ rd ∈ {0..30}, ∀ rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. encode_neon_umov([Reg({W|X}{rd}), RegLane(V{rn}, ts, i)]) = llvm-mc("umov {W|X}{rd}, V{rn}.{ts}[{i}]")
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i]
  domain: { rd: "0..=30", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]" }
  relation:
    op: eq
    lhs: encode_neon_umov([Reg(upper_gpr), RegLane(V{rn}, ts, i)])
    rhs: llvm_mc("umov {W|X}{rd}, V{rn}.{ts}[{i}]")
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_umov_neg_unsupported_elem_size
- Tier: 4
- Rationale: Sweep — documented error path neon.rs:478 rejects any elem_size outside {b,h,s,d}. The property asserts that invalid sizes {q,8b,16b,4h,empty,x} return Err, matching llvm-mc. This is the documented invalid domain, not a limitation on accepted input.
- Doc contract: neon.rs:478 "unsupported umov element size" — asserted fingerprint 45bf1914
- Seed: (none) — coverage_gaps sweep (file-level; manual arm audit)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ bad_ts ∈ {q,8b,16b,4h,"",x}, ∀ i ∈ [0,15]. llvm-mc rejects umov Wd, Vn.bad_ts[i] ⇒ encode_neon_umov([Wd, RegLane(Vn, bad_ts, i)]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, bad_ts, i]
  domain: { rd: "0..=31", rn: "0..=31", bad_ts: "{q,8b,16b,4h,empty,x}", i: "0..=15" }
  relation:
    op: holds
    expr: encode_neon_umov([Reg(Wd), RegLane(Vn, bad_ts, i)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  bad_ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:478
```

## encode_neon_umov_neg_non_lane_src
- Tier: 4
- Rationale: Sweep — documented operand comment neon.rs:467 "Second operand should be a RegLane (v0.b[0])" and the `_ => Err("umov: expected register lane operand")` arm. Covers Reg / RegArrangement / Imm as the second operand (arity-1 None was already in neg_arity_sp_fp).
- Doc contract: neon.rs:467 "Second operand should be a RegLane (v0.b[0])" — asserted fingerprint e26be721
- Seed: (none) — coverage_gaps sweep (file-level; manual arm audit)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)], ∀ src ∈ {Reg(v{rn}), RegArrangement(v{rn}.8b), Imm(i)}. llvm-mc rejects the corresponding assembly ⇒ encode_neon_umov([Reg(gpr), src]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_umov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ts, i, kind]
  domain: { rd: "0..=31", rn: "0..=31", ts: "{b,h,s,d}", i: "[0, imax(ts)]", kind: "{Reg,RegArrangement,Imm}" }
  relation:
    op: holds
    expr: encode_neon_umov([Reg(gpr), non_lane_src]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ts: { gen: string }
  i: { gen: int, min: 0, max: 15, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
evidence: neon.rs:467
```
