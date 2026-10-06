# Properties: encode_store

## encode_store_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree S-type store decoder). encode_s / encode_float_store / C.SW / encode_load rejected as same-job siblings (private packer / FP / compressed / loads).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_diff_imm_llvm_mc
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, off ∈ [-2048, 2047]. encode_store([Reg(rs2), Mem{rs1, off}], funct3(mn)) = Word(w) ∧ w = llvm-mc("mn rs2, off(rs1)")
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_store
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, off]
  domain: { mn: {sb,sh,sw,sd}, rs2: gpr, rs1: gpr, off: i12 }
  relation:
    op: eq
    lhs: encode_store([Reg(rs2), Mem{rs1, off}], funct3(mn))
    rhs: llvm_mc_word("mn rs2, off(rs1)")
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:488-492 sb/sh/sw/sd => encode_store; README.md:302 S-type sb,sh,sw,sd; llvm-mc -triple=riscv64 -show-encoding
```

## encode_store_s_type_fields
- Tier: 4
- Rationale: Weaker than differential; unpacks the S-type layout independently of llvm-mc using the RISC-V ISA / README layout (not a copy of encode_s). Complements the differential by asserting opcode, funct3, rs1, rs2, signed imm12 reconstruction.
- Doc contract: encoder/mod.rs:297 "S-type: imm[11:5] | rs2[24:20] | rs1[19:15] | funct3[14:12] | imm[4:0] | opcode[6:0]" — asserted fingerprint 46316d2a
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_i_type_fields
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ 0..31, off ∈ [-2048, 2047]. let w = encode_store([Reg(x{rs2}), Mem{x{rs1}, off}], funct3(mn)) in Word. unpack_s(w) = (OP_STORE=0b0100011, funct3(mn), rs1, rs2, off)
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_store
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, off]
  domain: { mn: {sb,sh,sw,sd}, rs2: 0..31, rs1: 0..31, off: i12 }
  relation:
    op: eq
    lhs: unpack_s(encode_store([Reg(x{rs2}), Mem{x{rs1}, off}], funct3(mn)))
    rhs: (0b0100011, funct3(mn), rs1, rs2, off)
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:297 S-type layout; README.md:354 S-type bit fields; encoder/mod.rs:339 OP_STORE
```

## encode_store_abi_xn_alias
- Tier: 3
- Rationale: Metamorphic: ABI names, xN, and fp=s0/x8 name the same GPR. Round-trip rejected (no decoder). Complements differential by checking the encoder's register-alias table independently of llvm-mc.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_abi_xn_alias
- Formal: ∀ n, m ∈ 0..31, off ∈ [-2048, 2047], mn ∈ {sb,sh,sw,sd}. encode_store([Reg(x{n}), Mem{x{m}, off}]) = encode_store([Reg(ABI[n]), Mem{ABI[m], off}]) ∧ (n=8 ⇒ encode_store([Reg("fp"), Mem{x{m}, off}]) equals the xN form) ∧ (m=8 ⇒ encode_store([Reg(x{n}), Mem{"fp", off}]) equals the xN form)
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_store
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, off, mn]
  domain: { n: 0..31, m: 0..31, off: i12, mn: {sb,sh,sw,sd} }
  relation:
    op: eq
    lhs: encode_store([Reg(x{n}), Mem{x{m}, off}], funct3(mn))
    rhs: encode_store([Reg(ABI[n]), Mem{ABI[m], off}], funct3(mn))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
evidence: parser.rs:22 GPR names include ABI aliases; fp is s0/x8
```

## encode_store_reloc_lo
- Tier: 4
- Rationale: Documented S-type store relocations are Lo12S / PcrelLo12S / TprelLo12S (encoder/mod.rs:71,77), not the I-type twins used for loads. Word must equal mn rs2, 0(rs1). Stronger differential of ELF reloc numbers vs llvm-mc is not a numeric word comparison (llvm-mc emits a fixup, not a resolved word). parse_reloc_modifier returns I-type Lo variants; encode_store must remap them for stores.
- Doc contract: encoder/mod.rs:77 "R_RISCV_LO12_S - for SW/SD (absolute low 12 bits, S-type)" — asserted fingerprint 7dd360d2
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_reloc_lo
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident. encode_store([Reg(rs2), MemSymbol{rs1, "%lo(s)"}]) = WordWithReloc{word = encode_store([Reg(rs2), Mem{rs1, 0}]), reloc_type = Lo12S, symbol = s, addend = 0} ∧ same with %pcrel_lo → PcrelLo12S ∧ %tprel_lo → TprelLo12S
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: failing
- Counterexample: encode_store([Reg("x0"), MemSymbol{base:"x0", symbol:"%pcrel_lo(foo)"}], funct3=0) reloc_type=PcrelLo12I (want PcrelLo12S); %lo -> Lo12I (want Lo12S); %tprel_lo -> TprelLo12I (want TprelLo12S)
- Bug report: pbt-out/bug_reports/encode_store_lo_reloc_i_type.md

```property
function: encoder.encode_store
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, s]
  domain: { mn: {sb,sh,sw,sd}, rs2: gpr, rs1: gpr, s: ident }
  relation:
    op: eq
    lhs: encode_store([Reg(rs2), MemSymbol{rs1, "%lo(s)"}]).reloc_type
    rhs: RelocType::Lo12S
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
evidence: encoder/mod.rs:71 R_RISCV_PCREL_LO12_S for SW/SD; encoder/mod.rs:77 R_RISCV_LO12_S for SW/SD; llvm-mc fixup_riscv_lo12_s / pcrel_lo12_s / tprel_lo12_s
```

## encode_store_neg_arity_fp
- Tier: 5
- Rationale: Negative/error contract: empty list, missing 2nd operand, FP rs2/base, and Imm/Csr/Fence/RoundingMode as 2nd operand must Err. llvm-mc rejects FP registers on integer stores and requires a memory operand. Stronger oracles do not apply to the invalid domain.
- Doc contract: base.rs:219 "store: expected memory operand" — asserted fingerprint 68c05089
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_arity_fp
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rd ∈ GPR, fp ∈ FPR, off ∈ [-2048, 2047], bad ∈ {Imm, Csr, FenceArg, RoundingMode}. encode_store([], f3) is Err ∧ encode_store([Reg(rd)], f3) is Err ∧ encode_store([Reg(fp), Mem{rd, off}], f3) is Err ∧ encode_store([Reg(rd), Mem{fp, off}], f3) is Err ∧ encode_store([Reg(rd), bad], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, fp, off, bad]
  domain: { mn: store_mnemonic, rd: gpr, fp: fpr, off: i12, bad: non_mem }
  relation:
    op: throws
    expr: encode_store([], funct3(mn))
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rd: { gen: string }
  fp: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  bad: { gen: oneof, values: ["Imm", "Csr", "FenceArg", "RoundingMode"] }
expected_error: String
evidence: base.rs:219 error string store expected memory operand; llvm-mc rejects FP rs2 and missing mem
```

## encode_store_neg_extra
- Tier: 5
- Rationale: llvm-mc rejects a third operand ("invalid operand for instruction"). encode_store has no operands.len() check, so extra operands are ignored. Negative-error contract from the independent assembler reference.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_extra
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, off ∈ [-2048, 2047], extra ∈ Operand. llvm-mc rejects "mn rs2, off(rs1)" with a trailing operand ⇒ encode_store([Reg(rs2), Mem{rs1, off}, extra], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: failing
- Counterexample: encode_store([Reg("x0"), Mem{base:"x0", offset:0}, Imm(0)], funct3=0) // sb x0, 0(x0), 0 -> Ok(Word(0x00000023))
- Bug report: pbt-out/bug_reports/encode_store_extra_operand.md

```property
function: encoder.encode_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, off, extra]
  domain: { mn: store_mnemonic, rs2: gpr, rs1: gpr, off: i12, extra: Operand }
  relation:
    op: throws
    expr: encode_store([Reg(rs2), Mem{rs1, off}, extra], funct3(mn))
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc rejects extra operand on sd x1, 0(x2), x3
```

## encode_store_neg_imm_oob
- Tier: 5
- Rationale: llvm-mc documents store offset as an integer in [-2048, 2047]. Values outside that range must be rejected. encode_s masks 12 bits so out-of-range immediates wrap. Documented bound sampled at 2048, -2049 and extremes.
- Doc contract: README.md:354 "S-type:  [imm[11:5]| rs2 | rs1 | funct3 | imm[4:0] | opcode]" — asserted fingerprint f1be316d
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_imm_oob
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, imm ∉ [-2048, 2047]. llvm-mc rejects "mn rs2, imm(rs1)" ⇒ encode_store([Reg(rs2), Mem{rs1, imm}], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: failing
- Counterexample: encode_store([Reg("x0"), Mem{base:"x0", offset:2048}], funct3=0) // sb x0, 2048(x0) -> Ok(Word(0x80000023)) wrapping to -2048
- Bug report: pbt-out/bug_reports/encode_store_imm_oob.md

```property
function: encoder.encode_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, imm]
  domain: { mn: store_mnemonic, rs2: gpr, rs1: gpr, imm: i64_outside_i12 }
  relation:
    op: throws
    expr: encode_store([Reg(rs2), Mem{rs1, imm}], funct3(mn))
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc store offset must be an integer in the range [-2048, 2047]
```

## encode_store_neg_hi_modifier
- Tier: 5
- Rationale: llvm-mc accepts only %lo/%pcrel_lo/%tprel_lo on store memory operands and rejects %hi/%pcrel_hi/%tprel_hi. encode_store remaps those Hi20 types to S-type Lo instead of returning Err.
- Doc contract: encoder/mod.rs:77 "R_RISCV_LO12_S - for SW/SD (absolute low 12 bits, S-type)" — asserted fingerprint 7dd360d2
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_hi_modifier
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects "mn rs2, hi(s)(rs1)" ⇒ encode_store([Reg(rs2), MemSymbol{rs1, "hi(s)"}], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: failing
- Counterexample: encode_store([Reg("x0"), MemSymbol{base:"x0", symbol:"%hi(foo)"}], funct3=0) -> Ok(WordWithReloc{word:35, reloc_type:Lo12S, symbol:"foo"})
- Bug report: pbt-out/bug_reports/encode_store_hi_modifier.md

```property
function: encoder.encode_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, s, hi]
  domain: { mn: store_mnemonic, rs2: gpr, rs1: gpr, s: ident, hi: hi_modifier }
  relation:
    op: throws
    expr: encode_store([Reg(rs2), MemSymbol{rs1, hi_form(s)}], funct3(mn))
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
  hi: { gen: oneof, values: ["hi", "pcrel_hi", "tprel_hi"] }
expected_error: String
evidence: llvm-mc rejects hi, pcrel_hi, tprel_hi modifiers on store mem operands
```

## encode_store_neg_other_modifier
- Tier: 5
- Rationale: Sweep of the MemSymbol `other => other` arm. llvm-mc allows only lo modifiers on store offsets; %got_pcrel_hi, %tls_ie_pcrel_hi, %tls_gd_pcrel_hi, %tprel_add, and a plain symbol must Err. Distinct from hi remapping because parse_reloc_modifier yields GotHi20/TlsGotHi20/TlsGdHi20/TprelAdd/PcrelHi20-plain which the match does not remap.
- Doc contract: encoder/mod.rs:77 "R_RISCV_LO12_S - for SW/SD (absolute low 12 bits, S-type)" — asserted fingerprint 7dd360d2
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_hi_modifier
- Formal: ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident, mod ∈ {got_pcrel_hi, tls_ie_pcrel_hi, tls_gd_pcrel_hi, tprel_add, plain}. llvm-mc rejects the corresponding store mem operand ⇒ encode_store([Reg(rs2), MemSymbol{rs1, form(mod,s)}], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_store_pbt.rs
- Status: failing
- Counterexample: encode_store([Reg("x0"), MemSymbol{base:"x0", symbol:"%got_pcrel_hi(foo)"}], funct3=0) -> Ok(WordWithReloc{word:35, reloc_type:GotHi20, symbol:"foo"})
- Bug report: pbt-out/bug_reports/encode_store_other_modifier.md

```property
function: encoder.encode_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, s, modname]
  domain: { mn: store_mnemonic, rs2: gpr, rs1: gpr, s: ident, modname: other_modifier }
  relation:
    op: throws
    expr: encode_store([Reg(rs2), MemSymbol{rs1, form(modname, s)}], funct3(mn))
generators:
  mn: { gen: oneof, values: ["sb", "sh", "sw", "sd"] }
  rs2: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
  modname: { gen: oneof, values: ["got_pcrel_hi", "tls_ie_pcrel_hi", "tls_gd_pcrel_hi", "tprel_add", "plain"] }
expected_error: String
evidence: llvm-mc store offset must be lo-modifier or integer in [-2048, 2047]
```
