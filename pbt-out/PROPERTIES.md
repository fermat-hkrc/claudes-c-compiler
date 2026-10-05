# Properties: encode_neon_zip_uzp

## encode_neon_zip_uzp_diff_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README.md:12 claims the assembler accepts the same textual assembly gas would consume; encoder/mod.rs:1-7 claims 32-bit AArch64 words. llvm-mc `-triple=aarch64 -show-encoding` is an independent assembler of the same GNU-style mnemonics. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree ZIP/UZP/TRN decoder). Sibling encode_neon_ext / encode_neon_tbl / encode_neon_tbx rejected (same-job gate: different ARM permute subclasses).
- Doc contract: neon.rs:1093 "Encode NEON UZP1/UZP2/ZIP1/ZIP2" — asserted fingerprint bb62b080
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:130 (valid-domain llvm-mc agreement for permute)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}. encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T], opc(mnemonic), false) = llvm-mc(mnemonic Vd.T, Vn.T, Vm.T) as little-endian u32
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_zip_uzp
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, t, rd, rn, rm]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, t: {8b,16b,4h,8h,2s,4s,2d}, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_neon_zip_uzp([RegArrangement(v{rd},t), RegArrangement(v{rn},t), RegArrangement(v{rm},t)], opc(mnemonic), false)
    rhs: llvm_mc_word("{mnemonic} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1093
```

## encode_neon_zip_uzp_metamorphic_rd_rn_rm
- Tier: 3
- Rationale: ARM permute encoding isolates Rd at bits[4:0], Rn at bits[9:5], Rm at bits[20:16] (format comments neon.rs:1103-1106). Stronger differential is P1; this metamorphic does not need llvm-mc and checks field packing independently.
- Doc contract: neon.rs:1105 "ZIP1: 0 Q 0 01110 size 0 Rm 0 011 10 Rn Rd  (op_bits=011)" — asserted fingerprint 4819b69b
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:208 (Rd/Rn/Rm isolation)
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,2d}, opc ∈ {001,010,011,101,110,111}, rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w(rd,rn,rm)=encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T], opc, false). (w(rd1,rn1,rm1) ⊕ w(rd2,rn1,rm1)) & ~0x1F = 0 ∧ w(rd2,rn1,rm1) & 0x1F = rd2 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn2,rm1)) & ~(0x1F<<5) = 0 ∧ (w(rd1,rn2,rm1)>>5) & 0x1F = rn2 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn1,rm2)) & ~(0x1F<<16) = 0 ∧ (w(rd1,rn1,rm2)>>16) & 0x1F = rm2
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_zip_uzp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [t, opc, rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, opc: {1,2,3,5,6,7}, rd1: 0..31, rd2: 0..31, rn1: 0..31, rn2: 0..31, rm1: 0..31, rm2: 0..31 }
  body: changing only Rd/Rn/Rm differs only in bits[4:0]/[9:5]/[20:16]
generators:
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  opc: { gen: oneof, items: [1, 2, 3, 5, 6, 7], type: u32 }
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1105
```

## encode_neon_zip_uzp_invariant_arm_fields
- Tier: 3
- Rationale: ARM Advanced SIMD permute layout (and neon.rs:1103-1106) fixes every field of a success-path word. Independent of the producing statement via llvm-mc KAT mapping. Weaker than P1 differential; still pins Q/size/opc packing if llvm-mc is unavailable.
- Doc contract: neon.rs:1103 "UZP1: 0 Q 0 01110 size 0 Rm 0 001 10 Rn Rd  (op_bits=001)" — asserted fingerprint d47de039
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:247 (ARM field invariant)
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,2d}, opc ∈ {001,010,011,101,110,111}, rd,rn,rm ∈ {0..31}. let w=encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T], opc, false). (w>>31)&1=0 ∧ (w>>30)&1=Q(T) ∧ (w>>24)&0x3F=0b001110 ∧ (w>>22)&3=size(T) ∧ (w>>21)&1=0 ∧ (w>>16)&0x1F=rm ∧ (w>>15)&1=0 ∧ (w>>12)&7=opc ∧ (w>>10)&3=0b10 ∧ (w>>5)&0x1F=rn ∧ w&0x1F=rd. Q/size: 8b=(0,00) 16b=(1,00) 4h=(0,01) 8h=(1,01) 2s=(0,10) 4s=(1,10) 2d=(1,11)
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_zip_uzp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [t, opc, rd, rn, rm]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, opc: {1,2,3,5,6,7}, rd: 0..31, rn: 0..31, rm: 0..31 }
  body: ARM permute fields of encode_neon_zip_uzp word match Q/size/opc/Rd/Rn/Rm packing
generators:
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  opc: { gen: oneof, items: [1, 2, 3, 5, 6, 7], type: u32 }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1103
```

## encode_neon_zip_uzp_neg_arity
- Tier: 2
- Rationale: neon.rs:1096 documents "uzp/zip requires 3 operands"; the check is `operands.len() < 3`. llvm-mc/gas reject arity 0..2. Documented error contract.
- Doc contract: neon.rs:1096 "uzp/zip requires 3 operands" — domain-restriction fingerprint f7c87a19
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:311 (arity Err)
- Formal: ∀ n ∈ {0,1,2}, mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}. encode_neon_zip_uzp(ops[0..n], opc(mnemonic), false) = Err ∧ llvm-mc(arity-n asm) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_zip_uzp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, mnemonic, t, rd, rn, rm]
  domain: { n: 0..2, mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, t: {8b,16b,4h,8h,2s,4s,2d}, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: throws
    expr: encode_neon_zip_uzp(take(ops, n), opc(mnemonic), false)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1096
```

## encode_neon_zip_uzp_neg_extra_operand
- Tier: 2
- Rationale: llvm-mc and gas reject a fourth operand on ZIP/UZP/TRN. README.md:12 gas-compatibility is the contract. The SUT check is only `len < 3`, so extra operands are a documented-invalid input that the public assembler path must reject.
- Doc contract: neon.rs:1096 "uzp/zip requires 3 operands" — domain-restriction fingerprint f7c87a19
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:344 (extra operand Err)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm,extra ∈ {0..31}. llvm-mc(mnemonic Vd.T,Vn.T,Vm.T,Vextra.T)=Err ⇒ encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T,Vextra.T], opc(mnemonic), false)=Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: failing
- Counterexample: encode_neon_zip_uzp([v0.8b, v0.8b, v0.8b, v0.8b], 0b011, false)
- Bug report: bug_reports/encode_neon_zip_uzp_extra_operand.md

```property
function: encoder.encode_neon_zip_uzp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, t, rd, rn, rm, extra]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, t: {8b,16b,4h,8h,2s,4s,2d}, rd: 0..31, rn: 0..31, rm: 0..31, extra: 0..31 }
  relation:
    op: throws
    expr: encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T,Vextra.T], opc(mnemonic), false)
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1096
```

## encode_neon_zip_uzp_neg_reserved_1d
- Tier: 2
- Rationale: ARM Advanced SIMD permute lists size:Q=11:0 (1D) as reserved; llvm-mc and gas reject `*.1d`. README.md:12 gas-compatibility is the contract. neon_arr_to_q_size accepts "1d"; the function does not document 1d as valid.
- Doc contract: neon.rs:1093 "Encode NEON UZP1/UZP2/ZIP1/ZIP2" — asserted fingerprint bb62b080
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:368 (invalid T Err)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, rd,rn,rm ∈ {0..31}. llvm-mc(mnemonic Vd.1d,Vn.1d,Vm.1d)=Err ⇒ encode_neon_zip_uzp([Vd.1d,Vn.1d,Vm.1d], opc(mnemonic), false)=Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: failing
- Counterexample: encode_neon_zip_uzp([v0.1d, v0.1d, v0.1d], 0b011, false)
- Bug report: bug_reports/encode_neon_zip_uzp_reserved_1d.md

```property
function: encoder.encode_neon_zip_uzp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, rd, rn, rm]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: throws
    expr: encode_neon_zip_uzp([Vd.1d,Vn.1d,Vm.1d], opc(mnemonic), false)
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1093
```

## encode_neon_zip_uzp_neg_mismatched_t
- Tier: 2
- Rationale: ARM permute requires matching arrangements on Vd, Vn, Vm. llvm-mc and gas reject mismatched T. README.md:12 gas-compatibility. The SUT discards source arrangements.
- Doc contract: neon.rs:1093 "Encode NEON UZP1/UZP2/ZIP1/ZIP2" — asserted fingerprint bb62b080
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:410 (mismatched T Err)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}. (Td≠Tn ∨ Td≠Tm) ⇒ llvm-mc(mnemonic Vd.Td,Vn.Tn,Vm.Tm)=Err ∧ encode_neon_zip_uzp([Vd.Td,Vn.Tn,Vm.Tm], opc(mnemonic), false)=Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: failing
- Counterexample: encode_neon_zip_uzp([v0.8b, v0.8b, v0.16b], 0b011, false)
- Bug report: bug_reports/encode_neon_zip_uzp_mismatched_t.md

```property
function: encoder.encode_neon_zip_uzp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, td, tn, tm, rd, rn, rm]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, td: valid_t, tn: valid_t, tm: valid_t, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: throws
    expr: encode_neon_zip_uzp([Vd.Td,Vn.Tn,Vm.Tm], opc(mnemonic), false)
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  td: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tn: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  tm: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1093
```

## encode_neon_zip_uzp_neg_gpr_or_bare
- Tier: 2
- Rationale: llvm-mc and gas reject GPR, bare V (no arrangement), and FP scalar (B/H/S/D/Q) operands for ZIP/UZP/TRN. README.md:12 gas-compatibility. get_neon_reg accepts Operand::Reg and parse_reg_num maps x/w/d/s/q/h/b. Dest as Operand::Reg fails via empty arrangement; source as Operand::Reg encodes.
- Doc contract: neon.rs:1093 "Encode NEON UZP1/UZP2/ZIP1/ZIP2" — asserted fingerprint bb62b080
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:439 (GPR/bare Err)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}, kind ∈ {gpr_x, gpr_w, bare_v, fp_d, fp_s}. llvm-mc(non-arranged asm)=Err ⇒ encode_neon_zip_uzp(non-arranged ops, opc(mnemonic), false)=Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: failing
- Counterexample: encode_neon_zip_uzp([v0.8b, Reg("v0"), v0.8b], 0b011, false)
- Bug report: bug_reports/encode_neon_zip_uzp_bare_src.md

```property
function: encoder.encode_neon_zip_uzp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, t, rd, rn, rm, kind]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, t: {8b,16b,4h,8h,2s,4s,2d}, rd: 0..31, rn: 0..31, rm: 0..31, kind: {gpr_x,gpr_w,bare_v,fp_d,fp_s} }
  relation:
    op: throws
    expr: encode_neon_zip_uzp(non_neon_ops(kind), opc(mnemonic), false)
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:1093
```

## encode_neon_zip_uzp_diff_alt_spellings
- Tier: 4
- Rationale: Sweep — uppercase V prefix with lowercase T is accepted by llvm-mc and parse_reg_num lowercases. Same differential contract as P1 on an alt-spelling generator.
- Doc contract: neon.rs:1093 "Encode NEON UZP1/UZP2/ZIP1/ZIP2" — asserted fingerprint bb62b080
- Seed: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:271 (alt-spellings)
- Formal: ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}. encode_neon_zip_uzp([V{rd}.T, V{rn}.T, V{rm}.T], opc(mnemonic), false) = llvm-mc(mnemonic V{rd}.T, V{rn}.T, V{rm}.T)
- Test file: src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_zip_uzp
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, t, rd, rn, rm]
  domain: { mnemonic: {zip1,zip2,uzp1,uzp2,trn1,trn2}, t: {8b,16b,4h,8h,2s,4s,2d}, rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_neon_zip_uzp([RegArrangement(V{rd},t), RegArrangement(V{rn},t), RegArrangement(V{rm},t)], opc(mnemonic), false)
    rhs: llvm_mc_word("{mnemonic} V{rd}.{T}, V{rn}.{T}, V{rm}.{T}")
generators:
  mnemonic: { gen: oneof, items: ["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"] }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1093
```
