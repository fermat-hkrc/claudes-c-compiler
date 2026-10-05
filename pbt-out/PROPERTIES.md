# Properties: encode_neon_across

## encode_neon_across_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the GNU-style assembly the SUT claims to accept (README.md:12). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree decoder). Sibling encode_neon_addv / encode_neon_across_long rejected (same-job gate: different opcodes/mnemonics). SUT-boundary: internal-helper; encode_instruction passes operands through unchanged for umaxv/uminv/smaxv/sminv (encoder/mod.rs:693-696). Mapping: [Reg(Vd_scalar), RegArrangement(Vn,T)] <-> `{mnem} Vd, Vn.T`.
- Doc contract: neon.rs:439 "Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV" — asserted fingerprint b465ed41
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:228 encode_neon_addv_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (v,T) ∈ {(b,8b),(b,16b),(h,4h),(h,8h),(s,4s)}, ∀ (mnem,U,opc) ∈ {(umaxv,1,0b01010),(uminv,1,0b11010),(smaxv,0,0b01010),(sminv,0,0b11010)}. encode_neon_across([Reg(v∥rd), RegArrangement(v∥rn, T)], U, opc) = llvm-mc("{mnem} {v}{rd}, v{rn}.{T}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, mnem, u_bit, opcode]
  domain: { rd: 0..31, rn: 0..31, (v,t): valid_across_vt, (mnem,u_bit,opcode): caller_triples }
  relation:
    op: eq
    lhs: encode_neon_across([Reg(v||rd), RegArrangement(v||rn, t)], u_bit, opcode)
    rhs: llvm_mc("{mnem} {v}{rd}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  mnem: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:439
```

## encode_neon_across_meta_rd_rn
- Tier: 4
- Rationale: ARM across-lanes packing places Rd in bits[4:0] and Rn in bits[9:5] (format comment neon.rs:441). Metamorphic isolation of those fields is independent of the producing `|` expression. Stronger differential is P1; this catches field-overlap bugs even if llvm-mc is unavailable.
- Doc contract: neon.rs:441 "Format: 0 Q U 01110 size 11000 opcode 10 Rn Rd" — asserted fingerprint a7586ffe
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:240 encode_neon_addv_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, ∀ valid (v,T), ∀ caller (U,opc). Let w11 = encode_neon_across([v∥rd1, v∥rn1.T], U, opc), w21 = encode_neon_across([v∥rd2, v∥rn1.T], U, opc), w12 = encode_neon_across([v∥rd1, v∥rn2.T], U, opc). Then (w11 ⊕ w21) ∧ ¬0x1F = 0 ∧ (w11 ∧ 0x1F) = rd1 ∧ (w21 ∧ 0x1F) = rd2 ∧ (w11 ⊕ w12) ∧ ¬(0x1F≪5) = 0 ∧ ((w11≫5) ∧ 0x1F) = rn1 ∧ ((w12≫5) ∧ 0x1F) = rn2
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, v, t, u_bit, opcode]
  domain: { rd1,rd2,rn1,rn2: 0..31, (v,t): valid_across_vt, (u_bit,opcode): caller_pairs }
  body: ((w11 ^ w21) & !0x1F) == 0 && (w11 & 0x1F) == rd1 && ((w11 ^ w12) & !(0x1F << 5)) == 0 && ((w11 >> 5) & 0x1F) == rn1
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:441
```

## encode_neon_across_inv_layout
- Tier: 4
- Rationale: Function docstring neon.rs:441 claims ARM layout `0 Q U 01110 size 11000 opcode 10 Rn Rd`. Independent reconstruction from ARM Advanced SIMD across-lanes (Q/size from T: 8b=(0,00), 16b=(1,00), 4h=(0,01), 8h=(1,01), 4s=(1,10); 2S and size=11 reserved) is a value invariant, not a copy of the producing `|` chain. Weaker than P1 differential.
- Doc contract: neon.rs:441 "Format: 0 Q U 01110 size 11000 opcode 10 Rn Rd" — asserted fingerprint a7586ffe
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:270 encode_neon_addv_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (v,T), ∀ caller (U,opc). encode_neon_across([Reg(v∥rd), RegArrangement(v∥rn,T)], U, opc) = (Q≪30) | (U≪29) | (0b01110≪24) | (size≪22) | (0b11000≪17) | (opc≪12) | (0b10≪10) | (rn≪5) | rd, with (Q,size) the ARM table for T
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, u_bit, opcode]
  domain: { rd,rn: 0..31, (v,t): valid_across_vt, (u_bit,opcode): caller_pairs }
  relation:
    op: eq
    lhs: encode_neon_across([Reg(v||rd), RegArrangement(v||rn, t)], u_bit, opcode)
    rhs: arm_across_word(rd, rn, t, u_bit, opcode)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:441
```

## encode_neon_across_meta_u_opcode
- Tier: 4
- Rationale: Docstring neon.rs:443-444 places U at bit 29 and opcode at bits[16:12]. Metamorphic: flipping only U must differ only in bit 29; changing only opcode must differ only in bits[16:12]. Independent of the producing shifts.
- Doc contract: neon.rs:443 "`u_bit`: 0 for signed, 1 for unsigned" — asserted fingerprint 56e000f4; neon.rs:444 "`opcode`: 5-bit opcode (bits 16-12)" — asserted fingerprint 43218dcd
- Seed: src/backend/arm/assembler/encoder/neon.rs:2667 encode_neon_across_long_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (v,T), ∀ opc1,opc2 ∈ {0b01010,0b11010}. Let w_u0 = encode_neon_across(..., 0, opc1), w_u1 = encode_neon_across(..., 1, opc1). Then (w_u0 ⊕ w_u1) = (1≪29) ∧ changing only opcode differs only in bits[16:12]
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, opcode1, opcode2]
  domain: { rd,rn: 0..31, (v,t): valid_across_vt, opcode1,opcode2: {0b01010,0b11010} }
  body: (w_u0 ^ w_u1) == (1 << 29) && ((w_op1 ^ w_op2) & !(0x1F << 12)) == 0 && ((w_op1 >> 12) & 0x1F) == opcode1
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  opcode1: { gen: oneof, items: [10, 26] }
  opcode2: { gen: oneof, items: [10, 26] }
evidence: neon.rs:443
```

## encode_neon_across_neg_arity
- Tier: 3
- Rationale: neon.rs:447 returns Err when operands.len() < 2 with message "NEON across-vector requires 2 operands". Negative/error contract on the documented under-arity domain.
- Doc contract: neon.rs:447 "NEON across-vector requires 2 operands" — domain-restriction fingerprint 1d8f2315
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:296 encode_neon_addv_neg_arity
- Formal: ∀ n ∈ {0,1}, ∀ rd ∈ {0..31}, ∀ valid (v,T), ∀ caller (U,opc). encode_neon_across(ops with n operands, U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, v, t, u_bit, opcode]
  domain: { n: 0..1, rd: 0..31, (v,t): valid_across_vt, (u_bit,opcode): caller_pairs }
  relation:
    op: throws
    expr: encode_neon_across(ops_len_n, u_bit, opcode)
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:447
```

## encode_neon_across_neg_extra
- Tier: 3
- Rationale: Docstring/arity message "requires 2 operands" plus llvm-mc/gas reject a third operand (`invalid operand for instruction`). The public assembler contract (README.md:12) therefore rejects extra operands. The check is `len < 2`, so extras currently pass — that is the finding. Domain is the documented 2-operand form, not the passing one.
- Doc contract: neon.rs:447 "NEON across-vector requires 2 operands" — domain-restriction fingerprint 1d8f2315
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:308 encode_neon_addv_neg_extra
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ valid (v,T), ∀ caller (mnem,U,opc). llvm-mc("{mnem} {v}{rd}, v{rn}.{T}, v{extra}.{T}") is Err ⇒ encode_neon_across([Vd, Vn.T, Vextra.T], U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, v=b, t=8b, mnem=umaxv, u_bit=1, opcode=0b01010; encode_neon_across([b0, v0.8b, v0.8b], 1, 0b01010) = Ok(Word(0x2e30a800))
- Bug report: pbt-out/bug_reports/encode_neon_across_extra_operand.md

```property
function: encoder.neon.encode_neon_across
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, v, t, mnem, u_bit, opcode]
  domain: { rd,rn,extra: 0..31, (v,t): valid_across_vt, (mnem,u_bit,opcode): caller_triples }
  body: llvm_mc_rejects(3-operand) => encode_neon_across(three_ops, u_bit, opcode).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  mnem: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:447
```

## encode_neon_across_neg_invalid_t
- Tier: 3
- Rationale: ARM Advanced SIMD across lanes reserves size:Q=10:0 (2S) and size=11 (1D/2D). llvm-mc/gas reject T ∉ {8b,16b,4h,8h,4s}. The function does not document those T as valid; neon_arr_to_q_size accepts 2s/1d/2d. Keep them in the generator (documented ARM domain, not the passing one).
- Doc contract: neon.rs:439 "Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV" — asserted fingerprint b465ed41 (ARM across-lanes T set; function does not list 2S/1D/2D)
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:327 encode_neon_addv_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, ∀ T ∈ {2s,1d,2d,4b,8d,2h,1s}, ∀ caller (mnem,U,opc). llvm-mc("{mnem} {dest}{rd}, v{rn}.{T}") is Err ⇒ encode_neon_across([Reg(dest∥rd), RegArrangement(v∥rn,T)], U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t=2s, mnem=umaxv, u_bit=1, opcode=0b01010; encode_neon_across([s0, v0.2s], 1, 0b01010) = Ok(Word(0x2eb0a800))
- Bug report: pbt-out/bug_reports/encode_neon_across_reserved_t.md

```property
function: encoder.neon.encode_neon_across
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, mnem, u_bit, opcode]
  domain: { rd,rn: 0..31, t: {2s,1d,2d,4b,8d,2h,1s}, (mnem,u_bit,opcode): caller_triples }
  body: llvm_mc_rejects(invalid T) => encode_neon_across(ops, u_bit, opcode).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["2s", "1d", "2d", "4b", "8d", "2h", "1s"] }
  mnem: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:439
```

## encode_neon_across_neg_dest
- Tier: 3
- Rationale: ARM/gas/llvm-mc require dest Bd/Hd/Sd matching T. GPR (x/w/sp), arranged Vd.T, dest-width mismatch (h vs 8b, …) are rejected by llvm-mc. get_neon_reg accepts Operand::Reg of any prefix and discards dest arrangement (`let (rd, _)`). Public dispatch passes dest through. Domain is the documented dest, not the passing one.
- Doc contract: neon.rs:439 "Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV" — asserted fingerprint b465ed41
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:351 encode_neon_addv_neg_dest
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (v,T), ∀ caller (mnem,U,opc), ∀ dest ∈ {GPR-x, GPR-w, sp, Vd.T arranged, mismatched scalar prefix}. llvm-mc rejects dest ⇒ encode_neon_across([bad_dest, Vn.T], U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, v=b, t=8b, mnem=umaxv, u_bit=1, opcode=0b01010, kind=0; encode_neon_across([x0, v0.8b], 1, 0b01010) = Ok(Word(0x2e30a800))
- Bug report: pbt-out/bug_reports/encode_neon_across_invalid_dest.md

```property
function: encoder.neon.encode_neon_across
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, mnem, u_bit, opcode, dest_kind]
  domain: { rd,rn: 0..31, (v,t): valid_across_vt, dest_kind: {x,w,sp,arranged,mismatch} }
  body: llvm_mc_rejects(bad dest) => encode_neon_across(ops, u_bit, opcode).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  mnem: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
  dest_kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:439
```

## encode_neon_across_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: parser/llvm-mc accept uppercase mnemonic and V/B/H/S prefixes (parse_reg_num lowercases). Differential vs llvm-mc on that documented spelling domain. Same mapping as P1.
- Doc contract: neon.rs:439 "Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV" — asserted fingerprint b465ed41
- Seed: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:411 encode_neon_addv_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (v,T), ∀ caller (mnem,U,opc). encode_neon_across([Reg(V_up∥rd), RegArrangement(V∥rn, T)], U, opc) = llvm-mc("{MNEM} {V}{rd}, V{rn}.{T}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, mnem, u_bit, opcode]
  domain: { rd,rn: 0..31, (v,t): valid_across_vt, (mnem,u_bit,opcode): caller_triples }
  relation:
    op: eq
    lhs: encode_neon_across([Reg(upper(v)||rd), RegArrangement(V||rn, t)], u_bit, opcode)
    rhs: llvm_mc("{MNEM} {V}{rd}, V{rn}.{T}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: string }
  t: { gen: string }
  mnem: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:439
```
