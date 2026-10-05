# Properties: encode_neon_cmp_zero

## encode_neon_cmp_zero_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the GNU-style assembly this encoder claims to accept. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree compare-zero decoder). Sibling encode_neon_float_cmp_zero / encode_neon_three_same / encode_neon_two_misc rejected (same-job gate: FP size map, register-register three-same, or different two-misc opcodes).
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs:166 llvm-mc differential; encode_neon_float_cmp_zero_pbt KAT/diff
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, (U,opc,mnem) ∈ {(0,0b01001,cmeq),(1,0b01000,cmge),(0,0b01000,cmgt),(1,0b01001,cmle),(0,0b01010,cmlt)}. encode_neon_cmp_zero([Vd.T, Vn.T], U, opc) = llvm-mc(mnem Vd.T, Vn.T, #0)
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode, mnem]
  domain: { rd: v0_v31, rn: v0_v31, t: {8b,16b,4h,8h,2s,4s,2d} }
  relation:
    op: eq
    lhs: "encode_neon_cmp_zero(&[arr(rd,t), arr(rn,t)], u, opcode)"
    rhs: "llvm_mc_word(&format!(\"{mnem} v{rd}.{t}, v{rn}.{t}, #0\"))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
evidence: README.md:12 README.md:227 neon.rs:186 encoder/mod.rs:566-666
```

## encode_neon_cmp_zero_meta_rd_rn_u
- Tier: 4
- Rationale: ARM two-misc layout isolates Rd at [4:0], Rn at [9:5], U at bit 29. Metamorphic: changing only one of those inputs must flip only that field. Weaker than differential; kept as an independent algebraic check that does not depend on llvm-mc.
- Doc contract: neon.rs:188 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_neon_not_pbt.rs encode_neon_not_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, T ∈ valid_T, U ∈ {0,1}, opc ∈ {0b01000,0b01001,0b01010}. let w(rd,rn,U)=encode_neon_cmp_zero([Vd.T,Vn.T],U,opc). (w(rd1,rn1,U) ⊕ w(rd2,rn1,U)) ∧ ¬0x1F = 0 ∧ w[4:0]=rd. (w(rd1,rn1,U) ⊕ w(rd1,rn2,U)) ∧ ¬(0x1F≪5) = 0 ∧ w[9:5]=rn. w(...,0) ⊕ w(...,1) = 1≪29
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, t, u, opcode]
  domain: { rd1: v0_v31, t: valid_T }
  relation:
    op: holds
    expr: "(w11 ^ w21) & !0x1Fu32 == 0 && (w11 ^ w12) & !(0x1Fu32 << 5) == 0 && (w_u0 ^ w_u1) == (1u32 << 29)"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 8, max: 10, type: u32 }
evidence: neon.rs:188 ARM ARM two-register miscellaneous
```

## encode_neon_cmp_zero_inv_layout
- Tier: 4
- Rationale: ARM ARM two-register miscellaneous compare-with-zero field layout is an exact structural invariant of every success-path word. Independent of llvm-mc (bit positions from the ARM encoding diagram quoted at neon.rs:188).
- Doc contract: neon.rs:188 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_neon_not_pbt.rs encode_neon_not_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ valid_T, U ∈ {0,1}, opc ∈ {0b01000,0b01001,0b01010}. let w=encode_neon_cmp_zero([Vd.T,Vn.T],U,opc). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=U ∧ w[28:24]=01110 ∧ w[23:22]=size(T) ∧ w[21:17]=10000 ∧ w[16:12]=opc ∧ w[11:10]=10 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode]
  domain: { rd: v0_v31, t: valid_T }
  relation:
    op: holds
    expr: "w >> 31 == 0 && (w >> 30) & 1 == q && (w >> 29) & 1 == u && (w >> 24) & 0x1F == 0b01110 && (w >> 22) & 3 == size && (w >> 17) & 0x1F == 0b10000 && (w >> 12) & 0x1F == opcode && (w >> 10) & 3 == 0b10 && (w >> 5) & 0x1F == rn && (w & 0x1F) == rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 8, max: 10, type: u32 }
evidence: neon.rs:188 ARM ARM C7.2 CMEQ/CMGE/CMGT/CMLE/CMLT (vector, zero)
```

## encode_neon_cmp_zero_neg_arity
- Tier: 3
- Rationale: Documented minimum arity. neon.rs:191 returns Err when operands.len() < 2. Negative/error contract from the function's own domain restriction.
- Doc contract: neon.rs:191 "NEON compare-zero requires at least 2 operands" — domain-restriction fingerprint f5f706b7
- Seed: encode_neon_not_pbt.rs encode_neon_not_neg_arity
- Formal: ∀ n ∈ {0,1}, ops with |ops|=n. encode_neon_cmp_zero(ops, U, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t, u, opcode]
  domain: { n: {0,1} }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&ops_of_len(n), u, opcode)"
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: neon.rs:191
```

## encode_neon_cmp_zero_neg_extra
- Tier: 3
- Rationale: gas/llvm-mc reject a fourth operand on `cmeq Vd.T, Vn.T, #0`. README.md:12 claims the assembler accepts the same textual assembly gas would consume. The helper is caller-reachable with operands passed through (mod.rs:566-666). Extra operand must Err. Function comment's "at least 2" is a minimum, not a license to ignore trailing operands.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs encode_neon_not_neg_extra
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ valid_T, (U,opc,mnem) ∈ cmp_zero_table. llvm-mc(mnem Vd.T, Vn.T, #0, Vextra.T) = Err ∧ encode_neon_cmp_zero([Vd.T, Vn.T, Imm(0), Vextra.T], U, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t=8b, insn=(0, 0b01001, cmeq) — cmeq v0.8b, v0.8b, #0, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_cmp_zero_extra_operand.md

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, u, opcode]
  domain: { rd: v0_v31, t: valid_T }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&[arr(rd,t), arr(rn,t), Operand::Imm(0), arr(extra,t)], u, opcode)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: README.md:12 llvm-mc rejects fourth operand
```

## encode_neon_cmp_zero_neg_mismatch_t
- Tier: 3
- Rationale: ARM/gas/llvm-mc require matching T on Vd and Vn. The helper discards arr_n. Mismatched T must Err.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs encode_neon_not_neg_mismatch_t
- Formal: ∀ rd,rn ∈ {0..31}, Td ≠ Tn both in valid_T. llvm-mc(cmeq Vd.Td, Vn.Tn, #0) = Err ∧ encode_neon_cmp_zero([Vd.Td, Vn.Tn], 0, 0b01001) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, td=16b, tn=8b — cmeq v0.16b, v0.8b, #0
- Bug report: pbt-out/bug_reports/encode_neon_cmp_zero_mismatch_t.md

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn]
  domain: { td != tn, both in valid_T }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&[arr(rd, td), arr(rn, tn)], 0, 0b01001)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tn: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: README.md:12 ARM matching-T; llvm-mc invalid operand
```

## encode_neon_cmp_zero_neg_reserved_1d
- Tier: 3
- Rationale: ARM ARM size:Q=11:0 is reserved for integer compare-to-zero; llvm-mc rejects .1d. neon_arr_to_q_size maps "1d" to (0, 0b11). Input is accepted by the API (RegArrangement) so it must Err, not encode a reserved encoding.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_three_same_pbt.rs cmeq v0.1d reserved
- Formal: ∀ rd,rn ∈ {0..31}, (U,opc) in cmp_zero_u_opc. llvm-mc(cmeq Vd.1d, Vn.1d, #0) = Err ∧ encode_neon_cmp_zero([Vd.1d, Vn.1d], U, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, u=0, opcode=8 — cmeq v0.1d, v0.1d, #0
- Bug report: pbt-out/bug_reports/encode_neon_cmp_zero_reserved_1d.md

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, u, opcode]
  domain: { rd: v0_v31, t: 1d }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&[arr(rd, \"1d\"), arr(rn, \"1d\")], u, opcode)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 8, max: 10, type: u32 }
expected_error: String
evidence: ARM ARM reserved size:Q=11:0; llvm-mc invalid operand for .1d
```

## encode_neon_cmp_zero_neg_gpr_bare_nonv
- Tier: 3
- Rationale: gas/llvm-mc require arranged V registers (vN.T). GPR (x/w), SP, bare v/d/s/q, and non-V prefixes on RegArrangement are invalid. parse_reg_num accepts x/w/d/s/q/v/h/b and sp→31, so the helper must still Err.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs encode_neon_not_neg_gpr_bare_sp
- Formal: ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, arranged-x-prefix, GPR-src, s-dest}. llvm-mc rejects ∧ encode_neon_cmp_zero(ops, 0, 0b01001) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t=8b, kind=4 — cmeq x0.8b, v0.8b, #0
- Bug report: pbt-out/bug_reports/encode_neon_cmp_zero_non_v_prefix.md

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, kind]
  domain: { kind: gpr_bare_nonv_kinds }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&ops_for(kind), 0, 0b01001)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  kind: { gen: int, min: 0, max: 7, type: u8 }
expected_error: String
evidence: README.md:12 llvm-mc invalid operand for GPR/bare/non-V
```

## encode_neon_cmp_zero_neg_invalid_t
- Tier: 3
- Rationale: Sweep — arrangements other than the ARM-legal set (and other than reserved 1d) must Err via neon_arr_to_q_size. llvm-mc rejects 4b/8d/2h/1s/32b/empty.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs encode_neon_not_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {4b,8d,2h,1s,32b,ε}. llvm-mc(cmeq Vd.T, Vn.T, #0) = Err ∧ encode_neon_cmp_zero([Vd.T, Vn.T], 0, 0b01001) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { t: invalid_T }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&[arr(rd, t), arr(rn, t)], 0, 0b01001)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["4b", "8d", "2h", "1s", "32b", ""] }
expected_error: String
evidence: neon.rs:54 unsupported NEON arrangement; llvm-mc invalid operand
```

## encode_neon_cmp_zero_neg_nonreg
- Tier: 3
- Rationale: Sweep — Imm/Mem dest or Imm src must Err via get_neon_reg's non-register arm.
- Doc contract: neon.rs:191 "NEON compare-zero requires at least 2 operands" — domain-restriction fingerprint f5f706b7
- Seed: encode_neon_not_pbt.rs gpr/bare
- Formal: ∀ kind ∈ {Imm dest, Imm src, Mem dest}. encode_neon_cmp_zero(ops, 0, 0b01001) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, kind]
  domain: { kind: {0,1,2} }
  relation:
    op: throws
    expr: "encode_neon_cmp_zero(&ops_nonreg(kind), 0, 0b01001)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  kind: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: neon.rs:19 expected NEON register
```

## encode_neon_cmp_zero_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — uppercase V prefix is accepted by parse_reg_num (to_lowercase) and by llvm-mc; encodings must agree.
- Doc contract: neon.rs:186 "Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc." — asserted fingerprint 7a9bf7c1
- Seed: encode_neon_not_pbt.rs encode_neon_not_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ valid_T, (U,opc,mnem) ∈ cmp_zero_table. encode_neon_cmp_zero([V{rd}.T, V{rn}.T], U, opc) = llvm-mc(mnem V{rd}.T, V{rn}.T, #0)
- Test file: src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_cmp_zero
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode, mnem]
  domain: { rd: v0_v31, t: valid_T }
  relation:
    op: eq
    lhs: "encode_neon_cmp_zero(&[Arr(V{rd},t), Arr(V{rn},t)], u, opcode)"
    rhs: "llvm_mc_word(&format!(\"{mnem} V{rd}.{t}, V{rn}.{t}, #0\"))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
evidence: README.md:12 parse_reg_num to_lowercase; llvm-mc accepts V
```
