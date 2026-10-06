# Properties: encode_vsetivli

## encode_vsetivli_diff_llvm_mc_named
- Tier: 4
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler, same job: assemble `vsetivli` to a 32-bit word). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no in-tree vsetivli decoder. encode_vsetvli / encode_vsetvl rejected as siblings — same-job gate fails (vsetvli bit31=0 and rs1; vsetvl register vtype). Wrapper encode_instruction passes operands through (mod.rs:945).
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" / "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]" — asserted fingerprint b371a479
- Seed: encode_vsetvli_pbt.rs named differential (generalized to vsetivli uimm AVL)
- Formal: ∀ rd ∈ {x0..x31} ∪ ABI ∪ {fp}, uimm ∈ 0..=31, sew ∈ {e8,e16,e32,e64}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. encode_vsetivli([Reg(rd),Imm(uimm),Symbol(sew),Symbol(lmul),Symbol(ta),Symbol(ma)]) = Word(w) ∧ w = llvm-mc("vsetivli rd, uimm, sew, lmul, ta, ma", triple=riscv64, mattr=+v)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, uimm, sew, lmul, ta, ma]
  domain: { rd: gpr_name, uimm: 0..=31, sew: {e8,e16,e32,e64}, lmul: {m1,m2,m4,m8,mf2,mf4,mf8}, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)])
    rhs: llvm_mc_word("vsetivli {rd}, {uimm}, {sew}, {lmul}, {ta}, {ma}")
generators:
  rd: { gen: string }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: vector.rs:57-58; encoder/mod.rs:945; assembler/README.md:14; llvm-mc -triple=riscv64 -mattr=+v
```

## encode_vsetivli_diff_llvm_mc_imm
- Tier: 4
- Rationale: llvm-mc accepts a raw 10-bit vtypei immediate 0..=1023 as the third operand (prints the decoded named form). parse_vtypei vector.rs:17-18 "Raw immediate: treat as pre-encoded vtypei value". rustdoc format line pins vtypei[9:0]. Domain includes bounds 0 and 1023. Same differential reference as p1.
- Doc contract: vector.rs:57-58 "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]" — asserted fingerprint b371a479
- Seed: encode_vsetvli_pbt.rs imm differential (11-bit domain narrowed to 10-bit vsetivli field)
- Formal: ∀ rd ∈ GPR names, uimm ∈ 0..=31, v ∈ 0..=1023. encode_vsetivli([Reg(rd),Imm(uimm),Imm(v)]) = Word(w) ∧ w = llvm-mc("vsetivli rd, uimm, v")
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, uimm, v]
  domain: { rd: gpr_name, uimm: 0..=31, v: 0..=1023 }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Imm(uimm), Imm(v)])
    rhs: llvm_mc_word("vsetivli {rd}, {uimm}, {v}")
generators:
  rd: { gen: string }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: int, min: 0, max: 1023, type: u32 }
evidence: vector.rs:17-18; vector.rs:57-58; llvm-mc accepts vtypei 0..=1023 for vsetivli
```

## encode_vsetivli_format_fields
- Tier: 3
- Rationale: Algebraic invariant from the rustdoc format line and RISC-V V 1.0: opcode=1010111, funct3=111, bits[31:30]=11, rd in [11:7], uimm in [19:15], vtypei in [29:20] packed as [ma][ta][sew][lmul]. Weaker than differential; kept as an llvm-mc-independent structural check. Round-trip rejected (no decoder).
- Doc contract: vector.rs:57-58 "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]" — asserted fingerprint b371a479
- Seed: encode_vsetvli_pbt.rs format_fields
- Formal: ∀ rd ∈ 0..31, uimm ∈ 0..31, sew ∈ {e8,e16,e32,e64}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. let w = encode_vsetivli(named). w[6:0]=1010111 ∧ w[14:12]=111 ∧ w[31:30]=11 ∧ w[11:7]=rd ∧ w[19:15]=uimm ∧ w[29:20]=(ma<<7)|(ta<<6)|(sew_enc<<3)|lmul_enc
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, uimm, sew, lmul, ta, ma]
  domain: { rd: 0..=31, uimm: 0..=31, sew: {e8,e16,e32,e64}, lmul: {m1,m2,m4,m8,mf2,mf4,mf8}, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: holds
    lhs: unpack_vsetivli(sut_word(named_ops(rd,uimm,sew,lmul,ta,ma)))
    rhs: opcode=OP_V and funct3=0b111 and bits31_30=0b11 and got_rd=rd and got_uimm=uimm and vtypei=isa_vtypei(sew,lmul,ta,ma)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: vector.rs:57-58; RISC-V V 1.0 vsetivli encoding
```

## encode_vsetivli_abi_xn_alias
- Tier: 3
- Rationale: Metamorphic: ABI names (zero/ra/sp/a0/fp/…) must encode the same word as xN for the same register number. Independent of llvm-mc. Stronger round-trip rejected (no decoder). Differential already covers ABI via gpr_name; this isolates the alias law.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: encode_vsetvli_pbt.rs abi_xn_alias
- Formal: ∀ n ∈ 0..31, uimm ∈ 0..31, sew,lmul,ta,ma. encode_vsetivli(Reg(xN), Imm(uimm), named vtype) = encode_vsetivli(Reg(ABI[n]), Imm(uimm), named vtype). If n=8, Reg("fp") equals Reg("x8").
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, uimm, sew, lmul, ta, ma]
  domain: { n: 0..=31, uimm: 0..=31, sew: named SEW, lmul: named LMUL, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word(named_ops(xn(n), uimm, sew, lmul, ta, ma))
    rhs: sut_word(named_ops(abi_name(n), uimm, sew, lmul, ta, ma))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: encoder/mod.rs:226-274 ABI and xN both map through reg_num; fp aliases s0/x8
```

## encode_vsetivli_field_isolation
- Tier: 3
- Rationale: Metamorphic isolation: changing rd must not alter non-rd bits; changing uimm must not alter non-uimm bits; changing vtype named fields must not alter opcode/funct3/rd/uimm. Independent of llvm-mc. Stronger oracles already used above.
- Doc contract: vector.rs:57-58 "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]" — asserted fingerprint b371a479
- Seed: encode_vsetvli_pbt.rs field_isolation
- Formal: ∀ rd_a, rd_b, uimm_a, uimm_b ∈ 0..31, sew_a, sew_b, lmul, ta, ma. let wa, wb = encode with rd_a vs rd_b (same other fields). wa & ~(0x1f<<7) = wb & ~(0x1f<<7) ∧ wa[11:7]=rd_a ∧ wb[11:7]=rd_b. Analogous for uimm bits[19:15] and vtypei bits[29:20].
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd_a, rd_b, uimm_a, uimm_b, sew_a, sew_b, lmul, ta, ma]
  domain: { rd_a: 0..=31, rd_b: 0..=31, uimm_a: 0..=31, uimm_b: 0..=31, sew_a: named SEW, sew_b: named SEW, lmul: named LMUL, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: holds
    lhs: isolation(rd, uimm, vtypei)
    rhs: true
generators:
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  uimm_a: { gen: int, min: 0, max: 31, type: u32 }
  uimm_b: { gen: int, min: 0, max: 31, type: u32 }
  sew_a: { gen: string }
  sew_b: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: vector.rs:57-58 field layout
```

## encode_vsetivli_neg_arity_fp
- Tier: 2
- Rationale: Negative/error contract from llvm-mc: too few operands ("too few operands for instruction") and FP/vector rd ("invalid operand for instruction") must Err. State machine / round-trip N/A. Differential on valid inputs does not reach these branches.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: encode_vsetvli_pbt.rs neg_arity_fp
- Formal: ∀ ops with |ops| ∈ {0,1,2} (missing vtypei or more). encode_vsetivli(ops) = Err. ∀ fp ∈ {f0..f31,v0..v31,fa0,ft0,fs0,fa7,ft11,v0,v31}. encode_vsetivli([Reg(fp),Imm(1),named vtype]) = Err.
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: failing
- Counterexample: encode_vsetivli([Reg("x0"), Imm(0)]) = Ok(Word(0xc0007057))
- Bug report: bug_reports/encode_vsetivli_arity_two.md

```property
function: encoder.encode_vsetivli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp]
  domain: { ops: arity 0..=2, fp: FP or vector register name }
  relation:
    op: throws
    lhs: encode_vsetivli(ops)
    rhs: Err
expected_error: String
generators:
  ops: { gen: list }
  fp: { gen: string }
evidence: llvm-mc rejects too few operands and FP/vector rd for vsetivli
```

## encode_vsetivli_neg_extra
- Tier: 2
- Rationale: llvm-mc rejects a seventh operand after a complete named vtype ("operand must be e[8|16|32|64|...],m[...],[ta|tu],[ma|mu]"). The public wrapper passes extra operands through. parse_vtypei currently walks all remaining operands and may accept or overwrite rather than Err.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: encode_vsetvli_pbt.rs neg_extra
- Formal: ∀ rd ∈ GPR names, uimm ∈ 0..=31, sew,lmul,ta,ma, extra ∈ Operand. encode_vsetivli([Reg(rd),Imm(uimm),Symbol(sew),Symbol(lmul),Symbol(ta),Symbol(ma), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: failing
- Counterexample: encode_vsetivli([Reg("x0"), Imm(0), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu"), Imm(0)]) = Ok(Word(0xc0007057))
- Bug report: bug_reports/encode_vsetivli_extra_operand.md

```property
function: encoder.encode_vsetivli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, uimm, sew, lmul, ta, ma, extra]
  domain: { rd: gpr_name, uimm: 0..=31, sew: named SEW, lmul: named LMUL, ta: {ta,tu}, ma: {ma,mu}, extra: Operand }
  relation:
    op: throws
    lhs: encode_vsetivli(named_ops ++ [extra])
    rhs: Err
expected_error: String
generators:
  rd: { gen: string }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
  extra: { gen: string }
evidence: llvm-mc rejects extra operands after a complete vsetivli
```

## encode_vsetivli_diff_llvm_mc_wide_sew
- Tier: 4
- Rationale: llvm-mc accepts SEW e128/e256/e512/e1024 (vsew=100/101/110/111). parse_vtypei only matches e8/e16/e32/e64. Differential on the documented llvm-mc named-vtype alphabet (closed enum includes wide SEW). Not a documented input-domain restriction on encode_vsetivli itself.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: encode_vsetvli_pbt.rs wide_sew
- Formal: ∀ rd ∈ GPR names, uimm ∈ 0..=31, sew ∈ {e128,e256,e512,e1024}, lmul, ta, ma. encode_vsetivli(named) = Word(w) ∧ w = llvm-mc("vsetivli rd, uimm, sew, lmul, ta, ma")
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: failing
- Counterexample: encode_vsetivli([Reg("x0"), Imm(0), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")]) = Err("unknown vtypei field: e128")
- Bug report: bug_reports/encode_vsetivli_wide_sew.md

```property
function: encoder.encode_vsetivli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, uimm, sew, lmul, ta, ma]
  domain: { rd: gpr_name, uimm: 0..=31, sew: {e128,e256,e512,e1024}, lmul: named LMUL, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word(named_ops(rd, uimm, sew, lmul, ta, ma))
    rhs: llvm_mc_word("vsetivli {rd}, {uimm}, {sew}, {lmul}, {ta}, {ma}")
generators:
  rd: { gen: string }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: llvm-mc accepts e128/e256/e512/e1024 for vsetivli; assembler/README.md:14 V extension
```

## encode_vsetivli_neg_uimm_oob
- Tier: 2
- Rationale: Strengthening round. llvm-mc rejects AVL outside [0, 31] ("immediate must be an integer in the range [0, 31]"). rustdoc names the field uimm[4:0]. SUT does `get_imm(...) as u32 & 0x1F` which silently truncates. Not a documented input-domain restriction that declares OOB invalid for the API — llvm-mc and the 5-bit field width are the contract. Domain includes bounds 32, -1, 63.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: llvm-mc error on vsetivli a0, 32, e32, m1, ta, ma
- Formal: ∀ rd ∈ GPR names, uimm ∉ 0..=31, sew,lmul,ta,ma. encode_vsetivli([Reg(rd),Imm(uimm),named vtype]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: failing
- Counterexample: encode_vsetivli([Reg("x0"), Imm(-1), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu")]) = Ok(Word(0xc00ff057))
- Bug report: bug_reports/encode_vsetivli_uimm_oob.md

```property
function: encoder.encode_vsetivli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, uimm, sew, lmul, ta, ma]
  domain: { rd: gpr_name, uimm: Z \ 0..=31, sew: named SEW, lmul: named LMUL, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: throws
    lhs: encode_vsetivli(named_ops(rd, uimm, sew, lmul, ta, ma))
    rhs: Err
expected_error: String
generators:
  rd: { gen: string }
  uimm: { gen: int, min: -128, max: 128, type: i64 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: llvm-mc rejects AVL outside [0, 31]; vector.rs:57 uimm[4:0]
```

## encode_vsetivli_neg_vtypei_imm_oob
- Tier: 2
- Rationale: Strengthening round. rustdoc format pins vtypei[9:0] (10 bits). llvm-mc accepts 0..=1023 and rejects 1024/2047. SUT parse_vtypei masks 0x7FF then encode_vsetivli masks 0x3FF, silently wrapping 1024 to 0.
- Doc contract: vector.rs:57-58 "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]" — asserted fingerprint b371a479
- Seed: llvm-mc error on vsetivli a0, 1, 1024
- Formal: ∀ rd ∈ GPR names, uimm ∈ 0..=31, v ∈ 1024..=2047. encode_vsetivli([Reg(rd),Imm(uimm),Imm(v)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: failing
- Counterexample: encode_vsetivli([Reg("x0"), Imm(0), Imm(1024)]) = Ok(Word(0xc0007057))
- Bug report: bug_reports/encode_vsetivli_vtypei_imm_oob.md

```property
function: encoder.encode_vsetivli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, uimm, v]
  domain: { rd: gpr_name, uimm: 0..=31, v: 1024..=2047 }
  relation:
    op: throws
    lhs: encode_vsetivli([Reg(rd), Imm(uimm), Imm(v)])
    rhs: Err
expected_error: String
generators:
  rd: { gen: string }
  uimm: { gen: int, min: 0, max: 31, type: u32 }
  v: { gen: int, min: 1024, max: 2047, type: u32 }
evidence: llvm-mc rejects vsetivli raw vtypei 1024; vector.rs:58 vtypei[9:0]
```

## encode_vsetivli_neg_fp
- Tier: 2
- Rationale: Contract-surface sweep. llvm-mc rejects FP/vector rd ("invalid operand for instruction"). The combined arity+fp property failed on arity-two first; this property isolates the FP/vector rd error path. get_reg returns Err for names that are not integer GPRs.
- Doc contract: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" — asserted fingerprint 622c6b6d
- Seed: encode_vsetivli_pbt.rs neg_arity_fp
- Formal: ∀ fp ∈ {f0..f31,v0..v31,fa0,ft0,fs0,fa7,ft11}. encode_vsetivli([Reg(fp),Imm(1),named vtype]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetivli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp]
  domain: { fp: FP or vector register name }
  relation:
    op: throws
    lhs: encode_vsetivli(named_ops(fp, 1, e32, m1, ta, ma))
    rhs: Err
expected_error: String
generators:
  fp: { gen: string }
evidence: llvm-mc rejects FP/vector rd for vsetivli; get_reg (mod.rs:427) requires an integer register
```
