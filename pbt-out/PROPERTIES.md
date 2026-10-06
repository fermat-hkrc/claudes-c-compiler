# Properties: encode_mrs

## encode_mrs_diff_named
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, the independent AArch64 assembler the README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree MRS decoder). Sibling encode_msr rejected (same-job gate: MSR is write / L=0 / reversed operands). Domain is the closed SUT named-sysreg table × Xt; llvm-mc success must agree on the word and llvm-mc rejection must be matched by SUT Err.
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20
- Seed: src/backend/arm/codegen/globals.rs:25 `mrs x0, tpidr_el0`; encode_dmb_pbt.rs llvm-mc differential
- Formal: ∀ name ∈ NAMED, xt ∈ Xt. let asm = "mrs "+xt+", "+name; let sut = encode_mrs([Reg(xt), Symbol(name)]); (llvm-mc(asm)=Ok(w) ∧ sut=Ok(Word(w))) ∨ (llvm-mc(asm)=Err ∧ sut=Err)
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: failing
- Counterexample: name = "oslar_el1", xt = "x0"; SUT encoded mrs x0, oslar_el1 as 0xd5301080, llvm-mc rejected (expected readable system register)
- Bug report: pbt-out/bug_reports/encode_mrs_oslar_el1_write_only.md

```property
function: encoder.system.encode_mrs
oracle: differential
predicate:
  quantifier: forall
  vars: [name, xt]
  domain: { name: NAMED, xt: Xt }
  body: (llvm_mc("mrs "+xt+", "+name)=Ok(w) and encode_mrs([Reg(xt), Symbol(name)])=Ok(Word(w))) or (llvm_mc(...) = Err and encode_mrs(...) = Err)
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr, lr, x31] }
evidence: README.md:12 gas-compatible textual assembly; encoder/mod.rs:971 mrs => encode_mrs; ARM ARM MRS Xt, sysreg
```

## encode_mrs_diff_named_word
- Tier: 2
- Rationale: Implication half of the named differential: when llvm-mc accepts a named sysreg the SUT word must match. Write-only names (llvm-mc Err) do not discharge this property; they fail encode_mrs_diff_named. Catches encoding-table typos that the combined property may shrink past (cntv_cval_el0).
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20
- Seed: encode_mrs_diff_named
- Formal: ∀ name ∈ NAMED, xt ∈ Xt. llvm-mc("mrs "+xt+", "+name)=Ok(w) ⇒ encode_mrs([Reg(xt), Symbol(name)])=Ok(Word(w))
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: failing
- Counterexample: name = "cntv_cval_el0", xt = "x0"; SUT Word(0xd53be380) vs llvm-mc Word(0xd53be340)
- Bug report: pbt-out/bug_reports/encode_mrs_cntv_cval_el0_encoding.md

```property
function: encoder.system.encode_mrs
oracle: differential
predicate:
  quantifier: forall
  vars: [name, xt]
  domain: { name: NAMED, xt: Xt }
  body: llvm_mc("mrs "+xt+", "+name)=Ok(w) implies encode_mrs([Reg(xt), Symbol(name)])=Ok(Word(w))
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr, lr, x31] }
evidence: README.md:12; ARM ARM CNTV_CVAL_EL0 is S3_3_C14_C3_2
```

## encode_mrs_diff_generic
- Tier: 2
- Rationale: Differential vs llvm-mc on the generic S<op0>_<op1>_C<CRn>_C<CRm>_<op2> form with fields inside the ARM encoding widths (op0 0..=3, op1 0..=7, CRn/CRm 0..=15, op2 0..=7). llvm-mc accepts this entire box (probed). Independent of the named table.
- Doc contract: system.rs:178 "Bits [20:19] = op0, supplied entirely by the sysreg encoding field." — asserted fingerprint 68fa2cc0
- Seed: encode_dmb_pbt.rs llvm-mc differential
- Formal: ∀ op0∈0..=3, op1∈0..=7, crn∈0..=15, crm∈0..=15, op2∈0..=7, rt∈0..=31. encode_mrs([Reg("x"+rt), Symbol("s"+op0+"_"+op1+"_c"+crn+"_c"+crm+"_"+op2)]) = Ok(Word(llvm-mc("mrs x"+rt+", s"+op0+"_"+op1+"_c"+crn+"_c"+crm+"_"+op2)))
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_mrs
oracle: differential
predicate:
  quantifier: forall
  vars: [op0, op1, crn, crm, op2, rt]
  domain: { op0: 0..=3, op1: 0..=7, crn: 0..=15, crm: 0..=15, op2: 0..=7, rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_mrs([Reg("x"+rt), Symbol(sform(op0,op1,crn,crm,op2))])
    rhs: Word(llvm_mc("mrs x"+rt+", "+sform(op0,op1,crn,crm,op2)))
generators:
  op0: { gen: int, min: 0, max: 3, type: u32 }
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12; ARM ARM MRS generic S-register form; llvm-mc accepts op0 0..=3 / op1 0..=7 / CRn CRm 0..=15 / op2 0..=7
```

## encode_mrs_diff_numbered
- Tier: 2
- Rationale: Differential vs llvm-mc on numbered debug/PMU families. Domain is the documented n ranges (dbg* 0..=15, pmev* 0..=30).
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20. Numbered family n-bounds are documented on the helper encode_mrs calls (system.rs:199 "if n <= 15", system.rs:221 "if n <= 30").
- Seed: encode_dmb_pbt.rs llvm-mc differential
- Formal: ∀ fam ∈ {dbgbcr,dbgbvr,dbgwcr,dbgwvr}, n∈0..=15, rt∈0..=31. encode_mrs([Reg(xt_name(rt)), Symbol(fam+n+"_el1")]) = Ok(Word(llvm-mc(...))). And ∀ fam ∈ {pmevcntr,pmevtyper}, n∈0..=30, rt∈0..=31. same agreement with `_el0` suffix.
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_mrs
oracle: differential
predicate:
  quantifier: forall
  vars: [fam, n, rt]
  domain: { fam: DBG_OR_PMU, n: family_range(fam), rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_mrs([Reg("x"+rt), Symbol(fam_name(fam,n))])
    rhs: Word(llvm_mc("mrs x"+rt+", "+fam_name(fam,n)))
generators:
  fam: { gen: oneof, items: ["dbgbcr","dbgbvr","dbgwcr","dbgwvr","pmevcntr","pmevtyper"] }
  n: { gen: int, min: 0, max: 30, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: system.rs:186-232 numbered family comments (dbgbcr n<=15, pmevcntr n<=30); llvm-mc accepts those ranges
```

## encode_mrs_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM MRS encoding: bits[31:21]=0b11010101001 (fixed group + L=1), bits[4:0]=Rt. Metamorphic isolation: two Xt values for the same sysreg differ only in bits[4:0].
- Doc contract: system.rs:177 "MRS encoding: 0xd520_0000 has L=1 (bit 21) for read." — asserted fingerprint 50168250
- Seed: encode_dmb_pbt.rs ARM layout invariant
- Formal: ∀ name ∈ NAMED, rt∈0..=31. encode_mrs([Reg(xt_name(rt)), Symbol(name)])=Ok(Word(w)) ⇒ (w>>21)=0b11010101001 ∧ (w&0x1F)=rt. ∀ rt1≠rt2. (w(rt1) xor w(rt2)) & ~0x1F = 0 ∧ (w(rt1) xor w(rt2)) ≠ 0
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_mrs
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [name, rt]
  domain: { name: NAMED, rt: 0..=31 }
  body: let Word(w)=encode_mrs([Reg("x"+rt), Symbol(name)]); (w>>21)==0b11010101001 and (w&0x1f)==rt
generators:
  name: { gen: oneof, items: NAMED }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM MRS 1101 0101 00 1 op0 op1 CRn CRm op2 Rt; system.rs:177 L=1 bit 21
```

## encode_mrs_meta_casefold
- Tier: 4
- Rationale: Algebraic metamorphic: encode_mrs lowercases the Symbol before matching, so any ASCII case-fold of a named sysreg must produce the same word (llvm-mc is case-insensitive on sysreg names).
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20. README.md:12 gas-compatible names.
- Seed: encode_dmb_pbt.rs Barrier vs Symbol / case-fold
- Formal: ∀ name ∈ NAMED, cased ∈ casefolds(name), rt∈0..=31. encode_mrs([Reg(xt_name(rt)), Symbol(cased)]) = encode_mrs([Reg(xt_name(rt)), Symbol(name.to_lowercase())])
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_mrs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [name, cased, rt]
  domain: { name: NAMED, cased: casefolds(name), rt: 0..=31 }
  relation:
    op: eq
    lhs: encode_mrs([Reg("x"+rt), Symbol(cased)])
    rhs: encode_mrs([Reg("x"+rt), Symbol(name)])
generators:
  name: { gen: oneof, items: NAMED }
  cased: { gen: map, of: name, fn: case_fold }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 gas-compatible; llvm-mc accepts NZCV / CurrentEL / mixed case
```

## encode_mrs_neg_extra
- Tier: 5
- Rationale: Negative/error contract from gas/llvm-mc: MRS takes exactly two operands. Extra trailing operands are rejected by llvm-mc (`invalid operand`). SUT currently ignores extras (only operands[0] and [1] are read). No documented exclusion of extra operands on encode_mrs.
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20 (two-operand form). Extra operands are not declared valid.
- Seed: encode_dmb_pbt.rs encode_dmb_neg_extra
- Formal: ∀ name ∈ NAMED, xt ∈ Xt, extra ∈ Operand. llvm-mc("mrs "+xt+", "+name+", "+extra)=Err ⇒ encode_mrs([Reg(xt), Symbol(name), extra])=Err
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: failing
- Counterexample: name = "sp_el0", xt = "x0", extra = Reg("x0"); encode_mrs Ok while llvm-mc rejects `mrs x0, sp_el0, x0`
- Bug report: pbt-out/bug_reports/encode_mrs_extra_operand.md

```property
function: encoder.system.encode_mrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, xt, extra]
  domain: { name: NAMED, xt: Xt, extra: Operand }
  relation:
    op: throws
    expr: encode_mrs([Reg(xt), Symbol(name), extra])
generators:
  name: { gen: oneof, items: NAMED }
  xt: { gen: oneof, items: [x0, x30, xzr] }
  extra: { gen: oneof, items: [Reg, Imm, Symbol, Cond, Barrier] }
expected_error: extra operand (llvm-mc rejects arity>2)
evidence: llvm-mc `mrs x0, nzcv, x1` → invalid operand; README.md:12 gas-compatible
```

## encode_mrs_neg_wrong_dest
- Tier: 5
- Rationale: Negative/error contract from ARM ARM / the function's own "MRS Xt" comment: destination is a 64-bit GPR. llvm-mc rejects W registers, SP/WSP, and FP/SIMD registers (`invalid operand`). get_reg discards is_64 and parse_reg_num accepts w/sp/d/s/q/v/h/b.
- Doc contract: system.rs:55 "MRS Xt, system_reg" — asserted fingerprint c8f0bb20 (Xt, not Wt/SP/FP)
- Seed: encode_dmb_pbt.rs encode_dmb_neg_wrong_kind; callers only emit x0
- Formal: ∀ dest ∈ {w0..w30, wzr, sp, wsp, d0, s0, q0, v0, h0, b0}, name ∈ NAMED. llvm-mc("mrs "+dest+", "+name)=Err ⇒ encode_mrs([Reg(dest), Symbol(name)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: failing
- Counterexample: dest = "w0", name = "sp_el0"; encode_mrs Ok(Word(0xd5384100)) while llvm-mc rejects `mrs w0, sp_el0`
- Bug report: pbt-out/bug_reports/encode_mrs_w_dest.md

```property
function: encoder.system.encode_mrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, name]
  domain: { dest: W_SP_FP, name: NAMED }
  relation:
    op: throws
    expr: encode_mrs([Reg(dest), Symbol(name)])
generators:
  dest: { gen: oneof, items: [w0, w30, wzr, sp, wsp, d0, s0, q0, v0, h0, b0] }
  name: { gen: oneof, items: NAMED }
expected_error: invalid operand (llvm-mc rejects non-Xt dest)
evidence: system.rs:55 "MRS Xt, system_reg"; llvm-mc rejects mrs w0/sp/d0, nzcv
```

## encode_mrs_neg_unknown_arity_oob
- Tier: 5
- Rationale: Negative/error contract. Unknown sysreg names, missing operands, generic fields outside ARM widths, and numbered families out of documented n range must Err when llvm-mc rejects them. SUT masks oob generic fields via sysreg_encoding instead of rejecting.
- Doc contract: system.rs:59 "mrs needs system register name" — domain-restriction fingerprint 637b5f4f (non-Symbol / missing second operand). Unknown names and oob fields are not declared valid.
- Seed: encode_dmb_pbt.rs encode_dmb_neg_unknown / encode_dmb_neg_empty
- Formal: ∀ bad ∈ UnknownName ∪ Empty ∪ MissingSysreg ∪ OobGeneric ∪ OobNumbered. llvm-mc(asm(bad))=Err ⇒ encode_mrs(ops(bad))=Err
- Test file: src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
- Status: failing
- Counterexample: kind = 3, oob_g = "s4_0_c1_c0_1", xt = "x0"; SUT encodes while llvm-mc rejects `mrs x0, s4_0_c1_c0_1`
- Bug report: pbt-out/bug_reports/encode_mrs_oob_generic.md

```property
function: encoder.system.encode_mrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: UnknownName | Empty | MissingSysreg | OobGeneric | OobNumbered }
  relation:
    op: throws
    expr: encode_mrs(ops(bad))
generators:
  bad: { gen: oneof, items: [unknown_name, empty, missing_sysreg, oob_generic, oob_numbered] }
expected_error: llvm-mc-rejected MRS form
evidence: system.rs:59 "mrs needs system register name"; parse_generic_sysreg unsupported system register; llvm-mc rejects s9_0_c1_c0_1 / s3_8_c1_c0_1 / dbgbcr16_el1 / mrs x0 / mrs
```
