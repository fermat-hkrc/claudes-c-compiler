# Properties: encode_neon_eor3

## encode_neon_eor3_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree EOR3 decoder). Sibling encode_neon_aes / encode_neon_logical rejected (same-job gate: AES is two-reg crypto 01001110; three-same EOR has no fourth register).
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs:158 (valid-domain llvm-mc agreement)
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. encode_neon_eor3([Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b]) = llvm-mc("eor3 Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b")
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg }
  relation:
    op: eq
    lhs: encode_neon_eor3([arr(rd,16b), arr(rn,16b), arr(rm,16b), arr(rk,16b)])
    rhs: llvm_mc("eor3 v{rd}.16b, v{rn}.16b, v{rm}.16b, v{rk}.16b")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1111 Encode NEON EOR3 Vd.16b form; encoder/mod.rs:776 eor3 dispatch; README.md:238 NEON crypto lists eor3
```

## encode_neon_eor3_metamorphic_rd_rn_rm_rk
- Tier: 4c
- Rationale: Weaker than differential; isolates each register field independently of llvm-mc. ARM / neon.rs:1120 place Rd[4:0], Rn[9:5], Rm[20:16], Rk[14:10].
- Doc contract: neon.rs:1120 "Encoding: 11001110 000 Rm 0 Rk(4:0) 00 Rn Rd" — asserted fingerprint 08d0c4ce
- Seed: encode_neon_zip_uzp_pbt.rs:206 (Rd/Rn/Rm field isolation)
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2,rk1,rk2 ∈ {0..31}. changing only Rd (resp. Rn, Rm, Rk) differs only in bits[4:0] (resp. [9:5], [20:16], [14:10]) and the field equals the new register number
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2, rk1, rk2]
  domain: { rd1: vreg, rd2: vreg, rn1: vreg, rn2: vreg, rm1: vreg, rm2: vreg, rk1: vreg, rk2: vreg }
  body: (w1111 xor w2111) & ~0x1F == 0 AND (w2111 & 0x1F) == rd2 AND (w1111 xor w1211) & ~(0x1F<<5) == 0 AND ((w1211>>5)&0x1F)==rn2 AND (w1111 xor w1121) & ~(0x1F<<16) == 0 AND ((w1121>>16)&0x1F)==rm2 AND (w1111 xor w1112) & ~(0x1F<<10) == 0 AND ((w1112>>10)&0x1F)==rk2
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  rk1: { gen: int, min: 0, max: 31, type: u32 }
  rk2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1120 ARM field placement Rd[4:0] Rn[9:5] Rm[20:16] Rk[14:10]
```

## encode_neon_eor3_invariant_arm_fields
- Tier: 4d
- Rationale: Structural ARM SHA3 EOR3 layout from neon.rs:1120 and ARM Cryptographic three-register SHA3. Weaker than differential/metamorphic; pins fixed opcode bits.
- Doc contract: neon.rs:1120 "Encoding: 11001110 000 Rm 0 Rk(4:0) 00 Rn Rd" — asserted fingerprint 08d0c4ce
- Seed: encode_neon_zip_uzp_pbt.rs:249 (ARM field invariant)
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. let w = encode_neon_eor3([Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b]). w[31:24]=11001110 ∧ w[23:21]=000 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:10]=rk ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg }
  body: ((w>>24)&0xFF)==0b11001110 AND ((w>>21)&7)==0 AND ((w>>16)&0x1F)==rm AND ((w>>15)&1)==0 AND ((w>>10)&0x1F)==rk AND ((w>>5)&0x1F)==rn AND (w&0x1F)==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1120; ARM Cryptographic three-register SHA3 EOR3
```

## encode_neon_eor3_neg_arity
- Tier: 4e
- Rationale: neon.rs:1114 "eor3 requires 4 operands" and llvm-mc reject arity 0..3. Documented error contract.
- Doc contract: neon.rs:1114 "eor3 requires 4 operands" — domain-restriction fingerprint 5415ed54
- Seed: encode_neon_zip_uzp_pbt.rs:277 (arity Err)
- Formal: ∀ n ∈ {0,1,2,3}. ∀ rd,rn,rm,rk ∈ {0..31}. encode_neon_eor3(first n of [Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, rk]
  domain: { n: arity0to3, rd: vreg, rn: vreg, rm: vreg, rk: vreg }
  relation:
    op: throws
    expr: encode_neon_eor3(take(n, [arr(rd,16b), arr(rn,16b), arr(rm,16b), arr(rk,16b)]))
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1114 eor3 requires 4 operands; llvm-mc too few operands
```

## encode_neon_eor3_neg_extra_operand
- Tier: 4e
- Rationale: gas/llvm-mc reject a fifth operand. README.md:12 gas-compatible. The SUT check is `len < 4`, so extras are currently ignored — keep the contract red if it fails.
- Doc contract: neon.rs:1114 "eor3 requires 4 operands" — domain-restriction fingerprint 5415ed54
- Seed: encode_neon_zip_uzp_pbt.rs:307 (extra operand Err)
- Formal: ∀ rd,rn,rm,rk,extra ∈ {0..31}. encode_neon_eor3([Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b, Vextra.16b]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: failing
- Counterexample: encode_neon_eor3([v0.16b, v0.16b, v0.16b, v0.16b, v0.16b]) -> Ok(Word(0xce000000))
- Bug report: pbt-out/bug_reports/encode_neon_eor3_extra_operand.md

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk, extra]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg, extra: vreg }
  relation:
    op: throws
    expr: encode_neon_eor3([arr(rd,16b), arr(rn,16b), arr(rm,16b), arr(rk,16b), arr(extra,16b)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1114 eor3 requires 4 operands; llvm-mc rejects fifth operand
```

## encode_neon_eor3_neg_invalid_t
- Tier: 4e
- Rationale: ARM SHA3 EOR3 and the function docstring name only .16B. llvm-mc rejects 8b/4h/8h/2s/4s/2d/1d. Arrangement is discarded by the SUT (`let (rd, _)`) — keep red if it encodes.
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs:331 (reserved 1d Err)
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. ∀ t ∈ {8b,4h,8h,2s,4s,2d,1d}. encode_neon_eor3([Vrd.t, Vrn.t, Vrm.t, Vrk.t]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: failing
- Counterexample: encode_neon_eor3([v0.8b, v0.8b, v0.8b, v0.8b]) -> Ok(Word(0xce000000))
- Bug report: pbt-out/bug_reports/encode_neon_eor3_invalid_t.md

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg, t: invalid_t }
  relation:
    op: throws
    expr: encode_neon_eor3([arr(rd,t), arr(rn,t), arr(rm,t), arr(rk,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "4h", "8h", "2s", "4s", "2d", "1d"] }
expected_error: String
evidence: neon.rs:1111 Encode NEON EOR3 Vd.16b form; ARM SHA3 EOR3 only 16B
```

## encode_neon_eor3_neg_mismatched_t
- Tier: 4e
- Rationale: llvm-mc/gas require matching .16B on all four operands. SUT discards arrangements so a mixed-T vector currently encodes — keep red if it does.
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs:350 (mismatched T Err)
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. ∀ td,tn,tm,tk ∈ {8b,16b,4h,8h,2s,4s,2d}. (∃ T=16b) ∧ (∃ T≠16b) ⇒ encode_neon_eor3([Vrd.td, Vrn.tn, Vrm.tm, Vrk.tk]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: failing
- Counterexample: encode_neon_eor3([v0.16b, v0.8b, v0.8b, v0.8b]) -> Ok(Word(0xce000000))
- Bug report: pbt-out/bug_reports/encode_neon_eor3_mismatched_t.md

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk, td, tn, tm, tk]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg, td: t_any, tn: t_any, tm: t_any, tk: t_any }
  body: (td,tn,tm,tk) != (16b,16b,16b,16b) ==> encode_neon_eor3([arr(rd,td), arr(rn,tn), arr(rm,tm), arr(rk,tk)]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tn: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tm: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tk: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
expected_error: String
evidence: neon.rs:1111 Encode NEON EOR3 Vd.16b form; llvm-mc rejects mismatched T
```

## encode_neon_eor3_neg_gpr_or_bare
- Tier: 4e
- Rationale: llvm-mc/gas require Vn.16B arrangement operands, not GPR/FP/bare-V/SP. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp to 31.
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs:375 (GPR/bare Err)
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. ∀ kind ∈ GPR-dest | bare-V-src | X-src | bare-V-dest | X.16b-dest. encode_neon_eor3(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: failing
- Counterexample: encode_neon_eor3([Reg("x0"), v0.16b, v0.16b, v0.16b]) -> Ok(Word(0xce000000))
- Bug report: pbt-out/bug_reports/encode_neon_eor3_gpr_or_bare.md

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk, kind, fp_prefix]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg, kind: 0..4, fp_prefix: x|w|d|s|q|h|b }
  relation:
    op: throws
    expr: encode_neon_eor3(ops_for_kind)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  fp_prefix: { gen: oneof, values: ["x", "w", "d", "s", "q", "h", "b"] }
expected_error: String
evidence: neon.rs:1111 Encode NEON EOR3 Vd.16b form; get_neon_reg accepts Operand::Reg
```

## encode_neon_eor3_diff_alt_spellings
- Tier: 2
- Rationale: Sweep: uppercase mnemonic/V/16B is accepted by llvm-mc and parse_reg_num lowercases prefixes. Differential vs llvm-mc on the alt-spelling domain.
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs alt-spellings sweep
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. encode_neon_eor3([Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b] with uppercase V names) = llvm-mc("EOR3 Vrd.16B, Vrn.16B, Vrm.16B, Vrk.16B")
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg }
  relation:
    op: eq
    lhs: encode_neon_eor3([arr_V(rd,16b), arr_V(rn,16b), arr_V(rm,16b), arr_V(rk,16b)])
    rhs: llvm_mc("EOR3 V{rd}.16B, V{rn}.16B, V{rm}.16B, V{rk}.16B")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1111 Encode NEON EOR3 Vd.16b form; parse_reg_num lowercases V prefix
```

## encode_neon_eor3_neg_nonreg
- Tier: 4e
- Rationale: Sweep of get_neon_reg error path: Imm/Mem/Label at any slot must Err. Documented by get_neon_reg `other => Err`.
- Doc contract: neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b" — asserted fingerprint 9cef8e80
- Seed: encode_neon_zip_uzp_pbt.rs GPR/non-reg rejection
- Formal: ∀ rd,rn,rm,rk ∈ {0..31}. ∀ slot ∈ {0,1,2,3}. ∀ kind ∈ {Imm, Mem, Label}. encode_neon_eor3(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_eor3
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rk, kind, slot]
  domain: { rd: vreg, rn: vreg, rm: vreg, rk: vreg, kind: 0..2, slot: 0..3 }
  relation:
    op: throws
    expr: encode_neon_eor3(ops_with_nonreg_at_slot)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rk: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 3, type: usize }
expected_error: String
evidence: neon.rs:19 get_neon_reg other => Err expected NEON register
```
