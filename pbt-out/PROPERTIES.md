# Properties: encode_neon_two_misc_narrow

## encode_neon_two_misc_narrow_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree XTN/SQXTN decoder). Sibling encode_neon_two_misc rejected (same-job gate: matching-T ABS/NEG vs narrowing extract). Sibling encode_neon_fcvtn rejected (FP convert vs integer extract). Sibling encode_neon_xtl rejected (widen vs narrow). README.md:12 claims gas-compatible textual assembly; llvm-mc -triple=aarch64 -show-encoding is an independent assembler of that contract.
- Doc contract: neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN" — asserted fingerprint e5258bf7
- Seed: encode_neon_fcvtn_pbt.rs:238 (valid-domain llvm-mc agreement)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (tb,ta,is_high) ∈ {(8b,8h,false),(16b,8h,true),(4h,4s,false),(8h,4s,true),(2s,2d,false),(4s,2d,true)}, ∀ (mnem,u,opc) ∈ {(xtn,0,0b10010),(sqxtn,0,0b10100),(uqxtn,1,0b10100),(sqxtun,1,0b10010)}. encode_neon_two_misc_narrow([RegArrangement(v||rd,tb), RegArrangement(v||rn,ta)], u, opc, is_high) = Word(llvm-mc(mnem||(is_high?"2":"") || " v"||rd||"."||tb||", v"||rn||"."||ta))
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pair, family]
  domain: { rd: "0..=31", rn: "0..=31", pair: "{(8b,8h,false),(16b,8h,true),(4h,4s,false),(8h,4s,true),(2s,2d,false),(4s,2d,true)}", family: "{(xtn,0,0b10010),(sqxtn,0,0b10100),(uqxtn,1,0b10100),(sqxtun,1,0b10010)}" }
  relation:
    op: eq
    lhs: encode_neon_two_misc_narrow([RegArrangement(v||rd,tb), RegArrangement(v||rn,ta)], u, opc, is_high)
    rhs: llvm_mc_word(mnem ++ suffix ++ " v" ++ rd ++ "." ++ tb ++ ", v" ++ rn ++ "." ++ ta)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["8b/8h/0", "16b/8h/1", "4h/4s/0", "8h/4s/1", "2s/2d/0", "4s/2d/1"] }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:202
```

## encode_neon_two_misc_narrow_meta_rd_rn_q_u_opcode
- Tier: 4
- Rationale: Algebraic metamorphic field isolation. Stronger differential is the primary property; this checks that Rd, Rn, Q, U, and opcode each occupy disjoint ARM fields (independent of llvm-mc). State machine / round-trip rejected as above.
- Doc contract: neon.rs:204 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_neon_fcvtn_pbt.rs:258 (Rd/Rn/Q isolation)
- Formal: ∀ valid two-misc-narrow operands. changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only is_high in bit 30; only u_bit in bit 29; only opcode in bits[16:12]
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, rn, rn2, pair, family]
  domain: { rd: "0..=31", rd2: "0..=31", rn: "0..=31", rn2: "0..=31", pair: pairs, family: families }
  body: changing only Rd xor-masks to bits[4:0]; only Rn to bits[9:5]; only is_high to bit 30; only U to bit 29; only opcode to bits[16:12]
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["8b/8h/0", "16b/8h/1", "4h/4s/0", "8h/4s/1", "2s/2d/0", "4s/2d/1"] }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:204
```

## encode_neon_two_misc_narrow_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM asimdmisc (Advanced SIMD two-register miscellaneous), confirmed by llvm-mc encodings (xtn v0.8b, v1.8h = 0x0e212820). Stronger differential is primary. The SUT comment restates the packing; the invariant asserts ARM/llvm-mc field layout including size from source Ta.
- Doc contract: neon.rs:204 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_neon_fcvtn_pbt.rs:287 (ARM field layout)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (tb,ta,is_high) ∈ valid pairs, ∀ (u,opc) ∈ {(0,0b10010),(0,0b10100),(1,0b10100),(1,0b10010)}. let w = encode_neon_two_misc_narrow(...). bit31(w)=0 ∧ bit30(w)=is_high ∧ bit29(w)=u ∧ bits[28:24](w)=01110 ∧ bits[23:22](w)=size(ta) ∧ bits[21:17](w)=10000 ∧ bits[16:12](w)=opc ∧ bits[11:10](w)=10 ∧ bits[9:5](w)=rn ∧ bits[4:0](w)=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, pair, family]
  domain: { rd: "0..=31", rn: "0..=31", pair: pairs, family: families }
  body: word bits match ARM asimdmisc 0 Q U 01110 size 10000 opcode 10 Rn Rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["8b/8h/0", "16b/8h/1", "4h/4s/0", "8h/4s/1", "2s/2d/0", "4s/2d/1"] }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:204
```

## encode_neon_two_misc_narrow_neg_arity
- Tier: 4
- Rationale: Negative/error contract. neon.rs:207 "NEON two-reg narrow requires 2 operands"; llvm-mc rejects arity 0..=1. Domain restriction on arity < 2.
- Doc contract: neon.rs:207 "NEON two-reg narrow requires 2 operands" — domain-restriction fingerprint 791f23e5
- Seed: encode_neon_fcvtn_pbt.rs:310
- Formal: ∀ n ∈ {0,1}, ∀ rd ∈ {0..31}, ∀ ta ∈ arrangements, ∀ is_high ∈ {false,true}, ∀ u ∈ {0,1}, ∀ opc ∈ {0b10010,0b10100}. encode_neon_two_misc_narrow(ops.take(n), u, opc, is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, ta, is_high, u, opc]
  domain: { n: "0..=1", rd: "0..=31", ta: arrangements, is_high: bool, u: "0..=1", opc: "{0b10010,0b10100}" }
  relation:
    op: holds
    expr: encode_neon_two_misc_narrow(ops.take(n), u, opc, is_high).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, choices: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  is_high: { gen: bool }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opc: { gen: oneof, choices: [18, 20] }
evidence: neon.rs:207
```

## encode_neon_two_misc_narrow_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. neon.rs:207 "NEON two-reg narrow requires 2 operands" is the documented arity; llvm-mc/gas reject a third operand. The body only checks len < 2, so extra operands are accepted — that is the finding, not a generator exclusion.
- Doc contract: neon.rs:207 "NEON two-reg narrow requires 2 operands" — asserted fingerprint 791f23e5
- Seed: encode_neon_fcvtn_pbt.rs:331
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ valid (tb,ta,is_high,u,opc,mnem). llvm-mc rejects mnem||suffix || " v"||rd||"."||tb||", v"||rn||"."||ta||", v"||extra||"."||tb ∧ encode_neon_two_misc_narrow([Vd,Vn,Vextra], u, opc, is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc_narrow([v0.8b, v0.8h, v0.8b], u_bit=0, opcode=0b10010, is_high=false)
- Bug report: bug_reports/encode_neon_two_misc_narrow_extra_operand.md

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, pair, family]
  domain: { rd: "0..=31", rn: "0..=31", extra: "0..=31", pair: pairs, family: families }
  relation:
    op: holds
    expr: encode_neon_two_misc_narrow([Vd, Vn, Vextra], u, opc, is_high).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["8b/8h/0", "16b/8h/1", "4h/4s/0", "8h/4s/1", "2s/2d/0", "4s/2d/1"] }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:207
```

## encode_neon_two_misc_narrow_neg_mismatched_tb_ta
- Tier: 4
- Rationale: Negative/error contract from ARM ARM XTN{2} Vd.Tb, Vn.Ta (Tb must match Ta and Q) and llvm-mc rejection of illegal arrangements. neon.rs:215 documents unsupported source arrangements; dest pairing is the public XTN contract named at neon.rs:202. Invalid Tb with a legal Ta is still ARM-illegal; dest arrangement is part of the documented XTN form.
- Doc contract: neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN" — asserted fingerprint e5258bf7
- Seed: encode_neon_fcvtn_pbt.rs:354
- Formal: ∀ rd,rn ∈ {0..31}, ∀ tb,ta ∈ arrangements, ∀ is_high ∈ {false,true}, ∀ family. ¬valid_pair(tb,ta,is_high) ⇒ llvm-mc rejects the asm ∧ encode_neon_two_misc_narrow([Vd.tb, Vn.ta], u, opc, is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc_narrow([v0.8h, v0.4s], u_bit=0, opcode=0b10010, is_high=false)
- Bug report: bug_reports/encode_neon_two_misc_narrow_mismatched_tb_ta.md

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, tb, ta, is_high, family]
  domain: { rd: "0..=31", rn: "0..=31", tb: arrangements, ta: arrangements, is_high: bool, family: families }
  relation:
    op: holds
    expr: "!valid_pair(tb,ta,is_high) => encode_neon_two_misc_narrow(...).is_err()"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, choices: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  ta: { gen: oneof, choices: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  is_high: { gen: bool }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:202
```

## encode_neon_two_misc_narrow_neg_gpr_or_bare
- Tier: 4
- Rationale: Negative/error contract. ARM XTN requires Vd.Tb / Vn.Ta arrangement registers. llvm-mc rejects GPR dest, bare v-reg without arrangement, and xN.Tb. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/v/h/b — those are still invalid XTN operands.
- Doc contract: neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN" — asserted fingerprint e5258bf7
- Seed: encode_neon_fcvtn_pbt.rs:382
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {gpr-dest, bare-src, bare-dest, x-arrangement-dest, gpr-src}. llvm-mc rejects the asm ∧ encode_neon_two_misc_narrow(ops(kind), 0, 0b10010, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc_narrow([Reg("v0"), v0.8h], u_bit=0, opcode=0b10010, is_high=false)
- Bug report: bug_reports/encode_neon_two_misc_narrow_bare_dest.md

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, fp_prefix]
  domain: { rd: "0..=31", rn: "0..=31", kind: "0..=4", fp_prefix: "{x,w,d,s,q,h,b}" }
  relation:
    op: holds
    expr: encode_neon_two_misc_narrow(ops(kind), 0, 0b10010, false).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  fp_prefix: { gen: oneof, choices: ["x", "w", "d", "s", "q", "h", "b"] }
evidence: neon.rs:202
```

## encode_neon_two_misc_narrow_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc on uppercase mnemonic and V-prefix registers. GNU as and llvm-mc accept XTN V0.8B, V1.8H. parse_reg_num lowercases the name. Complements the lowercase differential. Same rejection chain as the primary differential.
- Doc contract: neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN" — asserted fingerprint e5258bf7
- Seed: encode_neon_fcvtn_pbt.rs:447
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (tb,ta,is_high,family). encode_neon_two_misc_narrow([RegArrangement(V||rd,tb), RegArrangement(V||rn,ta)], u, opc, is_high) = Word(llvm-mc(MNEM||suffix || " V"||rd||"."||TB||", V"||rn||"."||TA))
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pair, family]
  domain: { rd: "0..=31", rn: "0..=31", pair: pairs, family: families }
  relation:
    op: eq
    lhs: encode_neon_two_misc_narrow([RegArrangement(V||rd,tb), RegArrangement(V||rn,ta)], u, opc, is_high)
    rhs: llvm_mc_word(MNEM ++ suffix ++ " V" ++ rd ++ "." ++ TB ++ ", V" ++ rn ++ "." ++ TA)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["8b/8h/0", "16b/8h/1", "4h/4s/0", "8h/4s/1", "2s/2d/0", "4s/2d/1"] }
  family: { gen: oneof, choices: ["xtn/0/18", "sqxtn/0/20", "uqxtn/1/20", "sqxtun/1/18"] }
evidence: neon.rs:202
```

## encode_neon_two_misc_narrow_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. neon.rs get_neon_reg returns Err for Imm/Mem/Label ("expected NEON register"). llvm-mc rejects `xtn #0, v0.8h`. Sweep property for the documented non-register error path.
- Doc contract: neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN" — asserted fingerprint e5258bf7
- Seed: encode_neon_fcvtn_pbt.rs:480
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {Imm,Mem,Label}, ∀ slot ∈ {0,1}. encode_neon_two_misc_narrow(ops with slot replaced by kind, 0, 0b10010, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_two_misc_narrow
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: "0..=31", rn: "0..=31", kind: "0..=2", slot: "0..=1" }
  relation:
    op: holds
    expr: encode_neon_two_misc_narrow(ops_with_nonreg, 0, 0b10010, false).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
evidence: neon.rs:202
```
