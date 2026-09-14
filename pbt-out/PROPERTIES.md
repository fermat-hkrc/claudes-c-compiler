# Properties: encode_neon_aes

## encode_neon_aes_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree AES decoder). Sibling encode_neon_rbit rejected (same-job gate fails: Advanced SIMD two-misc RBIT). Sibling encode_neon_eor3 rejected (SHA-3 EOR3). Sibling encode_neon_two_misc rejected (generic two-misc). x86 AES-NI rejected (different ISA). Doc evidence: README.md:11 gas-compatible assembler; README.md:238 NEON crypto aese/aesd/aesmc/aesimc; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:747-750 AES dispatch; ARM ARM Cryptographic AES Vd.16B, Vn.16B; neon.rs:1153-1157 purpose comment.
- Seed: neon.rs encode_neon_rbit_pbt llvm-mc differential
- Formal: ∀ rd,rn ∈ {0..31}, opc ∈ {00100,00101,00110,00111}, spell ∈ {lower,upper}. encode_neon_aes([RegArrangement(spell(v,rd),"16b"), RegArrangement(spell(v,rn),"16b")], opc) = Word(llvm-mc-aes(mnem(opc) Vd.16b, Vn.16b))
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_aes
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, opc, spell]
  domain:
    rd: "0..31"
    rn: "0..31"
    opc: "00100|00101|00110|00111"
    spell: "v|V and 16b|16B"
  relation:
    op: eq
    lhs: encode_neon_aes([RegArrangement(Vd,16b), RegArrangement(Vn,16b)], opc)
    rhs: llvm_mc_aes_word("mnem Vd.16b, Vn.16b")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  spell: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11; README.md:238; encoder/mod.rs:747-750; ARM ARM Cryptographic AES
```

## encode_neon_aes_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM Cryptographic AES field layout on the success path. Stronger differential covers agreement with llvm-mc; this pins the documented bit fields independently. Evidence: neon.rs:1153-1157 purpose comment; ARM ARM Cryptographic AES 01001110 size=00 1 01000 opcode 10 Rn Rd.
- Seed: neon.rs encode_neon_rbit_word_layout
- Formal: ∀ rd,rn ∈ {0..31}, opc ∈ {00100,00101,00110,00111}. let w = encode_neon_aes([Vd.16b, Vn.16b], opc). w[31:24]=01001110 ∧ w[23:17]=0010100 ∧ w[16:12]=opc ∧ w[11:10]=10 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_aes
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, opc]
  domain:
    rd: "0..31"
    rn: "0..31"
    opc: "00100|00101|00110|00111"
  relation:
    op: eq
    lhs: encode_neon_aes([RegArrangement(Vd,16b), RegArrangement(Vn,16b)], opc)
    rhs: "(0b01001110<<24)|(0b0010100<<17)|(opc<<12)|(0b10<<10)|(rn<<5)|rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: neon.rs:1153-1157; ARM ARM Cryptographic AES 01001110 size=00 1 01000 opcode 10 Rn Rd
```

## encode_neon_aes_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent field increments. Rd+1 / Rn+1 / AESE vs AESD each flip only the corresponding ARM field. Stronger differential already used; this catches field packing bugs that a single-word equality can miss. Evidence: ARM ARM Cryptographic AES Rd[4:0] Rn[9:5] opcode[16:12].
- Seed: neon.rs encode_neon_rbit_meta_rd_rn
- Formal: ∀ rd,rn ∈ {0..30}, opc ∈ {00100,00101,00110,00111}. encode(rd+1,rn,opc) = encode(rd,rn,opc)+1 ∧ encode(rd,rn+1,opc) = encode(rd,rn,opc)+(1<<5) ∧ encode(rd,rn,AESD) XOR encode(rd,rn,AESE) = 1<<12 ∧ encode(rd,rn,AESMC) XOR encode(rd,rn,AESE) = 1<<13
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_aes
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, opc]
  domain:
    rd: "0..30"
    rn: "0..30"
    opc: "00100|00101|00110|00111"
  relation:
    op: holds
    expr: "encode(rd+1)==encode(rd)+1 && encode(rn+1)==encode(rn)+(1<<5) && (encode_AESD ^ encode_AESE)==(1<<12) && (encode_AESMC ^ encode_AESE)==(1<<13)"
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Cryptographic AES Rd[4:0] Rn[9:5] opcode[16:12]
```

## encode_neon_aes_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects `aese v0.16b` / empty as too few operands; SUT documents "aes instruction requires 2 operands" at neon.rs:1147-1148. Evidence: llvm-mc error; assembler gas-compatibility (README.md:11).
- Seed: neon.rs encode_neon_rbit_neg_arity
- Formal: ∀ ops with |ops| < 2, opc ∈ AES opcodes. encode_neon_aes(ops, opc) is Err. ∀ non-register dest kind. encode_neon_aes([bad, Vn.16b], opc) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_aes
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, opc, which]
  domain:
    len: "0..1"
    opc: "00100|00101|00110|00111"
    which: "Imm|Mem|Shift|RegList|Label"
  relation:
    op: holds
    expr: "encode_neon_aes(ops[..len], opc).is_err() && encode_neon_aes([bad_dest, src], opc).is_err()"
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: neon.rs:1147-1148; llvm-mc "too few operands"; README.md:11
```

## encode_neon_aes_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand (`aese v0.16b, v1.16b, v2.16b` / `#0` / `[x0]`). Gas-compatible assembler (README.md:11) must reject extra operands. Documented bound: 2 operands.
- Seed: neon.rs encode_neon_rbit_neg_extra_operands
- Formal: ∀ rd,rn,extra ∈ {0..31}, opc ∈ AES opcodes, extra_kind ∈ {RegArrangement, Imm, Reg, Mem}. encode_neon_aes([Vd.16b, Vn.16b, extra], opc) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, opc=4, extra_kind=0 — aese v0.16b, v0.16b, v0.16b → Ok(Word(0x4e284800))
- Bug report: pbt-out/bug_reports/encode_neon_aes_extra_operand.md

```property
function: encoder.neon.encode_neon_aes
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, opc, extra_kind]
  domain:
    rd: "0..31"
    extra_kind: "RegArrangement|Imm|Reg|Mem"
  relation:
    op: holds
    expr: "encode_neon_aes([Vd.16b, Vn.16b, extra], opc).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects 3rd operand; README.md:11; ARM ARM AES is a 2-operand instruction
```

## encode_neon_aes_neg_bad_arrangement
- Tier: 4
- Rationale: Negative/error contract. ARM ARM Cryptographic AES admits only Vd.16B, Vn.16B; llvm-mc rejects .8b/.4s/.8h/.2d/.4h/.2s/.1d and empty T. Documented bound: T=16b exactly.
- Seed: neon.rs encode_neon_rbit_neg_bad_arrangement
- Formal: ∀ rd,rn ∈ {0..31}, T ∉ {16b,16B}, opc ∈ AES opcodes. encode_neon_aes([Vd.T, Vn.T], opc) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", opc=4 — aese v0.8b, v0.8b → Ok(Word(0x4e284800))
- Bug report: pbt-out/bug_reports/encode_neon_aes_bad_arrangement.md

```property
function: encoder.neon.encode_neon_aes
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, opc]
  domain:
    t: "8b|4h|8h|2s|4s|2d|1d|8s|4b|empty|b|h|s"
    opc: "00100|00101|00110|00111"
  relation:
    op: holds
    expr: "encode_neon_aes([Vd.T, Vn.T], opc).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  opc: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: ARM ARM Cryptographic AES Vd.16B, Vn.16B; llvm-mc "invalid operand" for .8b/.4s
```

## encode_neon_aes_neg_mismatch_bare_invalid
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects mismatched T (`aese v0.16b, v1.8b`), bare Vn without arrangement (`aese v0, v1`), and invalid names (v32/foo/empty). Gas-compatible assembler must reject these.
- Seed: neon.rs encode_neon_rbit_neg_mismatch_nonreg_invalid / encode_neon_rbit_neg_bare_src / encode_neon_rbit_neg_invalid_name
- Formal: ∀ mismatched dest/src T, bare src Reg, invalid dest/src name. encode_neon_aes(...) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, tn="8b", opc=4 — aese v0.16b, v0.8b → Ok(Word(0x4e284800)); also bare src Reg("v0") → Ok (invalid names v32/foo/empty/v/v99/v-1 correctly Err)
- Bug report: pbt-out/bug_reports/encode_neon_aes_mismatch_arrangement.md

```property
function: encoder.neon.encode_neon_aes
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, bad, opc]
  domain:
    td: "16b"
    tn: "8b|16b|4h|8h|2s|4s with tn != td"
    bad: "v32|foo|empty|v|v99|v-1"
  relation:
    op: holds
    expr: "encode([Vd.td, Vn.tn], opc).is_err() && encode([Vd.16b, Reg(Vn)], opc).is_err() && encode([bad, Vn.16b], opc).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tn: { gen: string }
  bad: { gen: string }
  opc: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects mismatched T, bare Vn, invalid names; README.md:11
```

## encode_neon_aes_neg_prefix_sp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects non-V prefixes (x/w/d/s/q/h/b) and SP as AES operands. parse_reg_num maps those prefixes and sp->31, so this is the documented bound (V0-V31 only) plus the gas-compatibility contract.
- Seed: neon.rs encode_neon_rbit_neg_bad_prefix / encode_neon_rbit_neg_sp
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b}, rd,rn ∈ {0..31}, opc ∈ AES opcodes. encode_neon_aes([prefix+rd.16b, prefix+rn.16b], opc) is Err. ∀ SP in dest or src. encode_neon_aes is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, opc=4, prefix="x" — aese x0.16b, x0.16b → Ok(Word(0x4e284800)); also aese sp.16b, v0.16b → Ok with Rd=31
- Bug report: pbt-out/bug_reports/encode_neon_aes_non_v_prefix.md

```property
function: encoder.neon.encode_neon_aes
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, prefix, which, opc]
  domain:
    prefix: "x|w|d|s|q|h|b"
    which: "dest SP | src SP"
  relation:
    op: holds
    expr: "encode([prefix+rd.16b, prefix+rn.16b], opc).is_err() && encode(SP slot, opc).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: string }
  which: { gen: int, min: 0, max: 1, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for x0.16b and sp.16b; README.md:11; ARM ARM Vd.16B Vn.16B
```
