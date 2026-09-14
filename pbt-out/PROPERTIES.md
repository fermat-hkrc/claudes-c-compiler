# Properties: encode_cas

## encode_cas_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree CAS decoder). encode_swp rejected as differential sibling (different LSE class, size 111000). encode_ldar_stlr rejected (bit21=0, no Rs). Doc evidence: assembler README.md:11 (same textual assembly as gas); README.md:242 lists cas variants; encoder/mod.rs:920-922 dispatch; ARM ARM CAS encoding.
- Seed: load_store.rs encode_ldar_stlr_pbt llvm-mc differential
- Formal: ∀ mnemonic ∈ {cas,casa,casal,casl}×{W,X} ∪ {casb,casab,casalb,caslb,cash,casah,casalh,caslh}×{W}, rs,rt ∈ 0..31, rn ∈ 0..31. encode_cas(mnemonic, [Reg(Rs), Reg(Rt), Mem{base:Xn|SP, offset:0}]) = Word(w) ∧ w = llvm-mc("-mattr=+lse " + "mnemonic Rs, Rt, [Rn]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, rs, rt, rn]
  domain: { mnemonic: 12 cas* mnemonics with matching W/X, rs,rt,rn: 0..31 }
  relation:
    op: eq
    lhs: encode_cas(mnemonic, [Reg(Rs), Reg(Rt), Mem{Xn|SP, 0}])
    rhs: llvm_mc("-mattr=+lse", "mnemonic Rs, Rt, [Rn]")
generators:
  mnemonic: { gen: oneof, values: ["cas","casa","casal","casl","casb","casab","casalb","caslb","cash","casah","casalh","caslh"] }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; README.md:242; encoder/mod.rs:920-922; ARM ARM CAS
```

## encode_cas_arm_fields
- Tier: 4d
- Rationale: ARM ARM CAS encoding size 001000 1 L 1 Rs o0 11111 Rn Rt. Weaker than differential (already used). Invariant is the architectural field layout, unpacked independently of the producing statement.
- Seed: load_store.rs encode_ldar_stlr_roundtrip_arm_fields
- Formal: ∀ valid (mnemonic, rs, rt, rn). let w = encode_cas(...).Word. w[31:30]=size(mnemonic,Rs) ∧ w[29:24]=001000 ∧ w[23]=1 ∧ w[22]=L ∧ w[21]=1 ∧ w[20:16]=rs ∧ w[15]=o0 ∧ w[14:10]=11111 ∧ w[9:5]=rn ∧ w[4:0]=rt
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, rs, rt, rn]
  domain: { valid CAS W/X/byte/half domain }
  relation:
    op: holds
    expr: arm_cas_fields(encode_cas(mnemonic, ops))
generators:
  mnemonic: { gen: oneof, values: ["cas","casa","casal","casl","casb","casab","casalb","caslb","cash","casah","casalh","caslh"] }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM CAS encoding size 001000 1 L 1 Rs o0 11111 Rn Rt; assembler README.md:11
```

## encode_cas_metamorphic_regs_ao
- Tier: 4c
- Rationale: ARM encoding places Rt in bits[4:0], Rn in bits[9:5], Rs in bits[20:16]; L at bit 22; o0 at bit 15. Incrementing one register or flipping acquire/release must change only that field. Weaker than differential. Required metamorphic companion.
- Seed: load_store.rs encode_ldar_stlr_metamorphic_l_bit
- Formal: ∀ valid CAS with rs,rt,rn ∈ 0..30. encode_cas(...,rt+1,...) xor encode_cas(...,rt,...) has only bits[4:0] = rt+1; Rn+1 only bits[9:5]; Rs+1 only bits[20:16]; casa xor cas = 1<<22; casl xor cas = 1<<15; casal xor cas = (1<<22)|(1<<15); encode_cas("CAS", ...) = encode_cas("cas", ...).
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, rt, rn]
  domain: { rs,rt,rn in 0..30 }
  relation:
    op: holds
    expr: field_independence(encode_cas, rs, rt, rn) && (casa xor cas == 1<<22) && (casl xor cas == 1<<15) && encode_cas(Mem{off:0}) == encode_cas(Mem{off:0-omitted})
generators:
  rs: { gen: int, min: 0, max: 30, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM CAS Rt bits[4:0] Rn bits[9:5] Rs bits[20:16] L bit22 o0 bit15; ARM {,#0}
```

## encode_cas_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc and gas reject a fourth operand. SUT checks operands.len() < 3 only. Documented error contract: extra operands are invalid assembly.
- Seed: load_store.rs encode_ldar_stlr extra-operand regression
- Formal: ∀ valid 3-operand CAS ops, extra ∈ Operand. encode_cas(mnemonic, ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: v=0, rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2") — cas w0, w0, [x0], x2
- Bug report: pbt-out/bug_reports/encode_cas_extra_operand.md

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, rs, rt, rn, extra]
  domain: { valid 3-operand CAS, extra any Operand }
  relation:
    op: throws
    expr: encode_cas(mnemonic, [Rs, Rt, Mem, extra])
    error: String
generators:
  mnemonic: { gen: oneof, values: ["cas","casa","casb","cash"] }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc/gas reject extra CAS operands; README.md:11 gas contract
```

## encode_cas_neg_sp_zr_base
- Tier: 4e
- Rationale: ARM/gas/llvm-mc: Rs/Rt are integer ZR not SP; Rn is Xn|SP not W, not WSP, not XZR/x31. Invalid domain must Err.
- Seed: load_store.rs encode_ldar_stlr_neg SP/W-base
- Formal: ∀ mnemonic in cas*, kind ∈ {Rs=SP, Rt=SP, Rs=WSP, Rt=WSP, base=W, base=WSP, base=XZR, base=x31, base=wzr}. encode_cas(...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: v=0, n=0, kind=0, is_64=false — cas sp, w1, [x2]
- Bug report: pbt-out/bug_reports/encode_cas_sp_as_rs.md (related: encode_cas_xzr_as_base.md, encode_cas_w_base.md)

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, kind]
  domain: { SP/WSP as Rs/Rt; W/WSP/XZR/x31 as base }
  relation:
    op: throws
    expr: encode_cas(mnemonic, invalid_ops)
    error: String
generators:
  mnemonic: { gen: oneof, values: ["cas","casa","casal","casl","casb","cash"] }
  kind: { gen: int, min: 0, max: 8, type: u32 }
expected_error: String
evidence: ARM ARM CAS Rs/Rt=ZR Rn=SP; llvm-mc/gas reject SP as Rs/Rt and XZR/W as base
```

## encode_cas_neg_mixed_fp_xbyte
- Tier: 4e
- Rationale: llvm-mc/gas reject mixed W/X Rs/Rt, FP/SIMD prefixes, and casb/cash with X registers. Invalid domain must Err.
- Seed: neighbouring encode_*_pbt mixed-width / FP negatives
- Formal: ∀ kind ∈ {mixed W/X, FP prefix s/d/q/v/h/b, casb|cash with X}. encode_cas(...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: n=0, kind=0, fp='b' — cas x0, w0, [x1]
- Bug report: pbt-out/bug_reports/encode_cas_mixed_width.md (related: encode_cas_fp_reg.md, encode_cas_casb_x_reg.md)

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { mixed W/X; FP prefixes; casb/cash X-regs }
  relation:
    op: throws
    expr: encode_cas(invalid_mixed_or_fp_or_xbyte)
    error: String
generators:
  kind: { gen: int, min: 0, max: 10, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc/gas operand mismatch for mixed W/X, FP, casb X; README.md:11
```

## encode_cas_neg_arity_and_shape
- Tier: 4e
- Rationale: CAS requires exactly 3 operands with the third a no-offset (or optional #0) Mem of Xn|SP. Fewer operands and pre/post/register-offset/non-mem forms are rejected by gas/llvm-mc.
- Seed: load_store.rs encode_ldar_stlr_neg_arity_and_shape
- Formal: ∀ shape ∈ {0 ops, 1 op, 2 ops, Imm/Symbol/Cond third, MemPreIndex, MemPostIndex, MemRegOffset}. encode_cas("cas", ops(shape)) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape]
  domain: { arity < 3 or third operand not bare Mem }
  relation:
    op: throws
    expr: encode_cas("cas", ops(shape))
    error: String
generators:
  shape: { gen: int, min: 0, max: 8, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands" / "invalid operand"; gas "comma expected" / "invalid addressing mode"
```

## encode_cas_neg_nonzero_offset
- Tier: 4e
- Rationale: ARM ARM optional offset is only #0; gas: "the optional immediate offset can only be 0"; llvm-mc rejects any #imm. Nonzero Mem.offset must Err. Bounds sampled at ±1, ±8, i64::MIN/MAX.
- Seed: (none) — ARM {,#0} and gas error text
- Formal: ∀ mnemonic in cas*, rs,rt,rn ∈ 0..31, off ∈ Z\{0}. encode_cas(mnemonic, [Rs, Rt, Mem{base, offset:off}]) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: v=0, rs=0, rt=0, rn=0, is_64=false, off=-1 — cas w0, w0, [x0, #-1]
- Bug report: pbt-out/bug_reports/encode_cas_nonzero_offset.md

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, rs, rt, rn, off]
  domain: { off != 0 }
  relation:
    op: throws
    expr: encode_cas(mnemonic, [Rs, Rt, Mem{offset:off}])
    error: String
generators:
  mnemonic: { gen: oneof, values: ["cas","casb","cash","casa"] }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -8, max: 8, type: i64 }
expected_error: String
evidence: ARM ARM CAS {,#0}; gas "optional immediate offset can only be 0"; llvm-mc rejects #imm
```

## encode_cas_neg_invalid_name
- Tier: 4e
- Rationale: Sweep — get_reg / parse_reg_num reject unparsable names. Documented by parse_reg_num (prefix x/w and 0..=31) and get_reg error "invalid register". Stronger oracles do not apply to the invalid-name domain.
- Seed: neighbouring encode_bfxil_neg_invalid_name
- Formal: ∀ slot ∈ {Rs, Rt, base}, name ∈ {foo, x32, w32, empty, r0, x-1, 31}. encode_cas("cas", ops with that slot = name) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, name]
  domain: { unparsable register/base names }
  relation:
    op: throws
    expr: encode_cas("cas", ops_with_invalid_name(slot, name))
    error: String
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, values: ["foo","x32","w32","","r0","x-1","31"] }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num; get_reg invalid register
```

## encode_cas_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — encode_cas lowercases the mnemonic; llvm-mc accepts CAS/Cas/CASB. Same differential contract as encode_cas_diff_llvm_mc over case variants.
- Seed: neighbouring encode_bfxil_diff_alt_spellings
- Formal: ∀ valid (v, rs, rt, rn, is_64), mode ∈ {upper, first-upper, lower}. encode_cas(case(mnemonic, mode), ops) = llvm-mc(case(mnemonic, mode) + " Rs, Rt, [Rn]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_cas
oracle: differential
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64, mode]
  domain: { 12 cas* mnemonics, case variants, rs,rt,rn 0..31 }
  relation:
    op: eq
    lhs: encode_cas(cased_mnemonic, [Reg(Rs), Reg(Rt), Mem{Xn|SP, 0}])
    rhs: llvm_mc("-mattr=+lse", cased_asm)
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  mode: { gen: int, min: 0, max: 2, type: u32 }
evidence: load_store.rs:824 mnemonic.to_lowercase; llvm-mc accepts uppercase CAS
```
