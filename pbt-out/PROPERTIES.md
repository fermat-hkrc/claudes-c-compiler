# Properties: encode_msr

## encode_msr_diff_named
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, the independent AArch64 assembler the README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree MSR decoder). Sibling encode_mrs rejected (same-job gate: MRS is read / L=1 / reversed operands / no PSTATE immediate). Domain is the closed SUT named-sysreg table × Xt; llvm-mc success must agree on the word and llvm-mc rejection must be matched by SUT Err.
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_mrs_pbt.rs llvm-mc named differential; README.md:12
- Formal: ∀ name ∈ NAMED, xt ∈ Xt. let asm = "msr "+name+", "+xt; let sut = encode_msr([Symbol(name), Reg(xt)]); (llvm-mc(asm)=Ok(w) ∧ sut=Ok(Word(w))) ∨ (llvm-mc(asm)=Err ∧ sut=Err)
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: name = "cntv_cval_el0", xt = "x0"; SUT Word(0xd51be380) vs llvm-mc Word(0xd51be340)
- Bug report: pbt-out/bug_reports/encode_msr_cntv_cval_el0_encoding.md

```property
function: encoder.system.encode_msr
oracle: differential
predicate:
  quantifier: forall
  vars: [name, xt]
  domain: { name: NAMED, xt: Xt }
  body: (llvm_mc("msr "+name+", "+xt)=Ok(w) and encode_msr([Symbol(name), Reg(xt)])=Ok(Word(w))) or (llvm_mc(...) = Err and encode_msr(...) = Err)
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr, lr, x31] }
evidence: "system.rs:294 MSR register form; encoder/mod.rs:974 msr => encode_msr; README.md:12 gas-compatible textual assembly"
```

## encode_msr_diff_named_readonly
- Tier: 2
- Rationale: Implication half of the named differential: when llvm-mc rejects a named sysreg the SUT must Err. Encoding-table typos (llvm-mc Ok, SUT wrong word) fail encode_msr_diff_named instead. Isolates read-only names (oslsr_el1).
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_msr_diff_named
- Formal: ∀ name ∈ NAMED, xt ∈ Xt. llvm-mc("msr "+name+", "+xt)=Err ⇒ encode_msr([Symbol(name), Reg(xt)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: name = "oslsr_el1", xt = "x0"; SUT encoded 0xd5101180, llvm-mc rejected
- Bug report: pbt-out/bug_reports/encode_msr_oslsr_el1_read_only.md

```property
function: encoder.system.encode_msr
oracle: differential
predicate:
  quantifier: forall
  vars: [name, xt]
  domain: { name: NAMED, xt: Xt }
  body: llvm_mc("msr "+name+", "+xt)=Err implies encode_msr([Symbol(name), Reg(xt)])=Err
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr, lr, x31] }
evidence: "system.rs:294 MSR register form; OSLSR_EL1 is read-only"
```

## encode_msr_diff_generic
- Tier: 2
- Rationale: Differential vs llvm-mc on the generic S<op0>_<op1>_C<CRn>_C<CRm>_<op2> form with fields inside the ARM encoding widths (op0 0..=3, op1 0..=7, CRn/CRm 0..=15, op2 0..=7). llvm-mc accepts this entire box (probed). Independent of the named table. Required metamorphic/differential property.
- Doc contract: system.rs:384 "Bits [20:19] = op0, supplied entirely by the sysreg encoding field." — asserted fingerprint 68fa2cc0
- Seed: encode_mrs_pbt.rs encode_mrs_diff_generic
- Formal: ∀ op0∈0..=3, op1∈0..=7, crn∈0..=15, crm∈0..=15, op2∈0..=7, rt∈0..=31. encode_msr([Symbol("s"+op0+"_"+op1+"_c"+crn+"_c"+crm+"_"+op2), Reg(xt_name(rt))]) = Ok(Word(llvm-mc("msr s"+op0+"_"+op1+"_c"+crn+"_c"+crm+"_"+op2+", "+xt_name(rt))))
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_msr
oracle: differential
predicate:
  quantifier: forall
  vars: [op0, op1, crn, crm, op2, rt]
  domain: { op0: 0..=3, op1: 0..=7, crn: 0..=15, crm: 0..=15, op2: 0..=7, rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_msr([Symbol(sform(op0,op1,crn,crm,op2)), Reg(xt_name(rt))])
    rhs: Word(llvm_mc("msr "+sform(op0,op1,crn,crm,op2)+", "+xt_name(rt)))
generators:
  op0: { gen: int, min: 0, max: 3, type: u32 }
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: "system.rs:384 Bits [20:19] = op0; ARM ARM MSR generic S-register form"
```

## encode_msr_diff_numbered
- Tier: 2
- Rationale: Differential vs llvm-mc on numbered debug/PMU families. Domain is the documented n ranges (dbg* 0..=15, pmev* 0..=30).
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_mrs_pbt.rs encode_mrs_diff_numbered
- Formal: ∀ fam ∈ {dbgbcr,dbgbvr,dbgwcr,dbgwvr}, n∈0..=15, rt∈0..=31. encode_msr([Symbol(fam+n+"_el1"), Reg(xt_name(rt))]) = Ok(Word(llvm-mc(...))). And ∀ fam ∈ {pmevcntr,pmevtyper}, n∈0..=30, rt∈0..=31. same agreement with `_el0` suffix.
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_msr
oracle: differential
predicate:
  quantifier: forall
  vars: [fam, n, rt]
  domain: { fam: DBG_OR_PMU, n: family_range(fam), rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_msr([Symbol(fam_name(fam,n)), Reg(xt_name(rt))])
    rhs: Word(llvm_mc("msr "+fam_name(fam,n)+", "+xt_name(rt)))
generators:
  fam: { gen: oneof, items: ["dbgbcr","dbgbvr","dbgwcr","dbgwvr","pmevcntr","pmevtyper"] }
  n: { gen: int, min: 0, max: 30, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: "system.rs:294 MSR register form; system.rs:186-232 numbered family comments"
```

## encode_msr_diff_imm_pstate
- Tier: 2
- Rationale: Differential vs llvm-mc on the MSR-immediate PSTATE form unique to this symbol. Domain is {daifset, daifclr, spsel} × imm 0..=15 (ARM CRm width and llvm-mc range). Documented bounds sampled exactly (0 and 15 pinned by generator min/max).
- Doc contract: system.rs:268 "MSR (immediate): msr <pstatefield>, #imm" — asserted fingerprint 009607ca
- Seed: encode_mrs_pbt.rs llvm-mc differential (generalized to PSTATE immediate)
- Formal: ∀ field ∈ {daifset, daifclr, spsel}, imm∈0..=15. encode_msr([Symbol(field), Imm(imm)]) = Ok(Word(llvm-mc("msr "+field+", #"+imm)))
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_msr
oracle: differential
predicate:
  quantifier: forall
  vars: [field, imm]
  domain: { field: {daifset, daifclr, spsel}, imm: 0..=15 }
  relation:
    op: eq
    lhs: encode_msr([Symbol(field), Imm(imm)])
    rhs: Word(llvm_mc("msr "+field+", #"+imm))
generators:
  field: { gen: oneof, items: ["daifset","daifclr","spsel"] }
  imm: { gen: int, min: 0, max: 15, type: i64 }
evidence: "system.rs:268 MSR immediate pstatefield; system.rs:270 daifset/daifclr/spsel op1/op2"
```

## encode_msr_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM MSR (register) encoding: bits[31:21]=0b11010101000 (fixed group + L=0), bits[4:0]=Rt. Metamorphic isolation: two Xt values for the same sysreg differ only in bits[4:0]. Stronger differential already used on the success path; this pins the ARM field layout independently of llvm-mc.
- Doc contract: system.rs:383 "MSR encoding: 0xd500_0000 has L=0 (bit 21) for write." — asserted fingerprint 4d4576e5
- Seed: encode_mrs_pbt.rs encode_mrs_inv_arm_layout
- Formal: ∀ name ∈ NAMED, rt∈0..=31. encode_msr([Symbol(name), Reg(xt_name(rt))])=Ok(Word(w)) ⇒ (w>>21)=0b11010101000 ∧ (w&0x1F)=rt. ∀ rt1≠rt2. (w(rt1) xor w(rt2)) & ~0x1F = 0 ∧ (w(rt1) xor w(rt2)) ≠ 0
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_msr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [name, rt1, rt2]
  domain: { name: NAMED, rt1: 0..=31, rt2: 0..=31 }
  body: let w1=encode_msr([Symbol(name), Reg(xt_name(rt1))]); let w2=encode_msr([Symbol(name), Reg(xt_name(rt2))]); (w1>>21)=0b11010101000 and (w1&0x1F)=rt1 and ((w1 xor w2) & ~0x1F)=0
generators:
  name: { gen: oneof, items: NAMED }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
evidence: "system.rs:383 MSR encoding 0xd500_0000 has L=0 for write"
```

## encode_msr_meta_casefold
- Tier: 4
- Rationale: Algebraic metamorphic: ASCII case-fold of a named sysreg is encoding-invariant (SUT lowercases; llvm-mc / gas accept mixed case). Weaker than differential but independently evidenced by the to_lowercase call and GNU assembler case-insensitivity.
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_mrs_pbt.rs encode_mrs_meta_casefold
- Formal: ∀ name ∈ NAMED, cased = casefold(name), rt∈0..=31. encode_msr([Symbol(cased), Reg(xt_name(rt))]) = encode_msr([Symbol(name.to_lowercase()), Reg(xt_name(rt))])
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_msr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [name, cased, rt]
  domain: { name: NAMED, cased: casefold(name), rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_msr([Symbol(cased), Reg(xt_name(rt))])
    rhs: encode_msr([Symbol(name.to_lowercase()), Reg(xt_name(rt))])
generators:
  name: { gen: oneof, items: NAMED }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: "system.rs:294 MSR register form; system.rs:264 to_lowercase"
```

## encode_msr_neg_extra
- Tier: 4
- Rationale: Negative/error contract from llvm-mc/gas: extra operands are invalid. encode_msr does not document extra operands as valid; README gas-compatibility requires rejection. Stronger differential already covers the 2-operand success path.
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_mrs_pbt.rs encode_mrs_neg_extra
- Formal: ∀ name ∈ NAMED, xt ∈ Xt, extra ∈ ExtraOperand. llvm-mc("msr "+name+", "+xt+", "+extra)=Err ⇒ encode_msr([Symbol(name), Reg(xt), extra])=Err
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: name = "sp_el0", xt = "x0", extra = Reg("x0"); SUT Ok(Word) while llvm-mc rejects
- Bug report: pbt-out/bug_reports/encode_msr_extra_operand.md

```property
function: encoder.system.encode_msr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, xt, extra]
  domain: { name: NAMED, xt: Xt, extra: ExtraOperand }
  body: encode_msr([Symbol(name), Reg(xt), extra]) = Err
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr, lr, x31] }
  extra: { gen: oneof, items: [Reg, Imm, Symbol, Cond, Barrier, Label] }
expected_error: String
evidence: "system.rs:294 MSR register two-operand form; llvm-mc rejects extra operands"
```

## encode_msr_neg_wrong_src_unknown_arity_oob
- Tier: 4
- Rationale: Negative/error contract. Non-Xt second operand, unknown names, omitted operands, out-of-range generic S-form, out-of-range numbered families, and PSTATE imm outside 0..=15 are rejected by llvm-mc. encode_msr comments do not declare those inputs valid.
- Doc contract: system.rs:265 "msr needs system register name" — domain-restriction fingerprint 1cc385e5
- Seed: encode_mrs_pbt.rs encode_mrs_neg_wrong_dest / encode_mrs_neg_unknown_arity_oob
- Formal: ∀ dest ∉ Xt, name ∈ NAMED. encode_msr([Symbol(name), Reg(dest)])=Err. ∀ unknown ∉ NAMED∪S-form∪numbered. encode_msr([Symbol(unknown), Reg(x0)])=Err. encode_msr([])=Err. encode_msr([Symbol(name)])=Err. ∀ oob S-form/numbered. encode_msr(...)=Err. ∀ field ∈ {daifset,daifclr,spsel}, imm ∉ 0..=15. encode_msr([Symbol(field), Imm(imm)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: kind = 4, oob_g = "s4_0_c1_c0_1", xt = "x0"; SUT encodes (op0 masked to 0) while llvm-mc rejects
- Bug report: pbt-out/bug_reports/encode_msr_oob_generic.md

```property
function: encoder.system.encode_msr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, dest, unknown, oob_g, oob_n, field, imm]
  domain: { kind: 0..=6, dest: WrongXt, unknown: UnknownName, oob_g: OobGeneric, oob_n: OobNumbered, field: PSTATE, imm: OobImm }
  body: encode_msr(ops_for(kind)) = Err
generators:
  kind: { gen: int, min: 0, max: 6, type: u8 }
  dest: { gen: oneof, items: [w0, wzr, sp, wsp, d0, s0, q0, v0] }
  unknown: { gen: string }
  oob_g: { gen: string }
  oob_n: { gen: string }
  field: { gen: oneof, items: ["daifset","daifclr","spsel"] }
  imm: { gen: int, min: -32, max: 32, type: i64 }
expected_error: String
evidence: "system.rs:265 msr needs system register name; ARM ARM Xt and CRm 0..=15"
```

## encode_msr_neg_wrong_src
- Tier: 4
- Rationale: Strengthening split of the combined negative property. Non-Xt second operand (Wt/SP/FP) is rejected by llvm-mc; MSR requires Xt. encode_msr discards is_64 from get_reg and accepts any parse_reg_num name.
- Doc contract: system.rs:294 "MSR (register): msr sysreg, Xt" — asserted fingerprint bfb88aaf
- Seed: encode_msr_neg_wrong_src_unknown_arity_oob kind 0
- Formal: ∀ dest ∉ Xt, name ∈ NAMED. llvm-mc("msr "+name+", "+dest)=Err ⇒ encode_msr([Symbol(name), Reg(dest)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: dest = "w0", name = "sp_el0"; SUT Ok(Word(0xd5184100)) while llvm-mc rejects
- Bug report: pbt-out/bug_reports/encode_msr_w_src.md

```property
function: encoder.system.encode_msr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, name]
  domain: { dest: WrongXt, name: NAMED }
  body: encode_msr([Symbol(name), Reg(dest)]) = Err
generators:
  dest: { gen: oneof, items: [w0, wzr, sp, wsp, d0, s0] }
  name: { gen: oneof, items: NAMED }
expected_error: String
evidence: "system.rs:294 MSR register Xt; ARM ARM Xt is a 64-bit GPR not SP"
```

## encode_msr_neg_oob_imm
- Tier: 4
- Rationale: Strengthening split. PSTATE CRm immediate is documented as bits[11:8] (4 bits); llvm-mc requires integer in range [0, 15]. encode_msr masks with & 0xF instead of rejecting.
- Doc contract: system.rs:269 "Encoding: 1101_0101_0000_0 op1[18:16] 0100 CRm[11:8] op2[7:5] 11111[4:0]" — asserted fingerprint 27999f64
- Seed: encode_msr_neg_wrong_src_unknown_arity_oob kind 6
- Formal: ∀ field ∈ {daifset, daifclr, spsel}, imm ∉ 0..=15. llvm-mc("msr "+field+", #"+imm)=Err ⇒ encode_msr([Symbol(field), Imm(imm)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_msr_pbt.rs
- Status: failing
- Counterexample: field = "daifset", imm = -2; SUT Ok (masked CRm) while llvm-mc rejects
- Bug report: pbt-out/bug_reports/encode_msr_oob_imm.md

```property
function: encoder.system.encode_msr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [field, imm]
  domain: { field: {daifset, daifclr, spsel}, imm: OobImm }
  body: encode_msr([Symbol(field), Imm(imm)]) = Err
generators:
  field: { gen: oneof, items: ["daifset","daifclr","spsel"] }
  imm: { gen: int, min: -32, max: 32, type: i64 }
expected_error: String
evidence: "system.rs:269 CRm[11:8] encoding; llvm-mc immediate must be an integer in range [0, 15]"
```
