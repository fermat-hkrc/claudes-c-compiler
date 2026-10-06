# Properties: encode_load

## encode_load_diff_imm_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler the SUT claims to replace (README.md:6-10 in-process assembler, no host `as`). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree I-type load decoder). encode_i / encode_float_load / C.LW / encode_store rejected as primary differential (same-job gate: private packer / FP / compressed / stores). Mapping: `[Reg(rd), Mem{base, offset}]` <-> `mn rd, offset(rs1)` for mn in {lb,lh,lw,ld,lbu,lhu,lwu}.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_jalr_pbt.rs:237 encode_jalr_kat_llvm_mc_x0_x1
- Formal: ∀ mn ∈ {lb,lh,lw,ld,lbu,lhu,lwu}, ∀ rd,rs1 ∈ GPR, ∀ off ∈ [-2048,2047]. encode_load([Reg(rd), Mem{rs1, off}], funct3(mn)) = Word(w) ∧ llvm-mc("mn rd, off(rs1)") = w
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, off]
  domain: { mn: {lb,lh,lw,ld,lbu,lhu,lwu}, rd: gpr, rs1: gpr, off: i12 }
  relation:
    op: eq
    lhs: encode_load([Reg(rd), Mem{rs1, off}], funct3(mn))
    rhs: llvm_mc("mn rd, off(rs1)")
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: README.md:300-301 I-type loads; encoder/mod.rs:478-484 dispatch; RISC-V ISA I-type LOAD opcode=0000011
```

## encode_load_i_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented I-type layout (README.md:353, encoder/mod.rs:289). Stronger differential is the sibling property; this unpacks opcode/rd/funct3/rs1/imm independently of llvm-mc so a mapping bug cannot hide a field packing bug.
- Doc contract: encoder/mod.rs:289 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: encode_jalr_pbt.rs unpack_i
- Formal: ∀ mn, rd, rs1, off∈[-2048,2047]. encode_load([Reg(rd), Mem{rs1,off}], f3) = Word(w) ⇒ opcode(w)=0b0000011 ∧ rd(w)=n(rd) ∧ funct3(w)=f3 ∧ rs1(w)=n(rs1) ∧ sext12(w)=off
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, off]
  domain: { mn: load_mnemonics, rd: gpr, rs1: gpr, off: i12 }
  body: unpack_i(encode_load(...)) == (OP_LOAD, n(rd), funct3(mn), n(rs1), off)
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: README.md:353 I-type layout; encoder/mod.rs:289 I-type field comment; encoder/mod.rs:336 OP_LOAD
```

## encode_load_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic alias invariance: ABI names, xN, and fp=s0/x8 are the same GPRs. Stronger differential already covers xN/ABI vs llvm-mc; this pins alias equality without depending on the reference.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint (none on encode_load; alias is GPR encoding)
- Seed: encode_jalr_pbt.rs ABI vs xN
- Formal: ∀ mn, n_rd, n_rs1 ∈ 0..31, ∀ off∈[-2048,2047]. encode_load(xN names) = encode_load(ABI names) = encode_load(fp when n=8)
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n_rd, n_rs1, off]
  domain: { mn: load_mnemonics, n_rd: 0..31, n_rs1: 0..31, off: i12 }
  relation:
    op: eq
    lhs: encode_load(xN)
    rhs: encode_load(ABI)
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  n_rd: { gen: int, min: 0, max: 31, type: u32 }
  n_rs1: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:156 reg_num ABI and xN; fp is s0/x8
```

## encode_load_reloc_lo
- Tier: 5
- Rationale: Documented load-type reloc mapping (base.rs:157 "Use Lo12I for load-type relocations"; encoder/mod.rs:67/73). llvm-mc accepts %lo/%pcrel_lo/%tprel_lo on loads and rejects %hi. Word must equal `mn rd, 0(rs1)` with the matching RelocType and extracted symbol.
- Doc contract: base.rs:157 "Use Lo12I for load-type relocations" — asserted fingerprint 1c7049c6
- Seed: encode_jalr_pbt.rs reloc lo
- Formal: ∀ mn, rd, rs1, s, mod ∈ {%lo,%pcrel_lo,%tprel_lo}. encode_load([Reg(rd), MemSymbol{rs1, "%mod(s)"}]) = WordWithReloc{word=encode_load(Mem{rs1,0}), reloc_type=Lo12I|PcrelLo12I|TprelLo12I, symbol=s, addend=0}
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, s, modifier]
  domain: { modifier: lo_pcrel_lo_tprel_lo }
  relation:
    op: holds
    expr: reloc.word == load_imm0 && reloc_type_matches(modifier) && reloc.symbol == s && reloc.addend == 0
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  s: { gen: string, type: String }
  modifier: { gen: oneof, options: [lo, pcrel_lo, tprel_lo] }
evidence: base.rs:157-165 reloc remap; encoder/mod.rs:67 PcrelLo12I; encoder/mod.rs:73 Lo12I
```

## encode_load_bare_symbol_pseudo
- Tier: 5
- Rationale: Documented pseudo expansion (base.rs:173-175): `ld rd, symbol` → auipc rd, %pcrel_hi(symbol); ld rd, 0(rd) with PCREL_HI20 + PCREL_LO12_I. llvm-mc emits the same two-instruction sequence. Words must match llvm-mc encodings of `auipc rd, 0` and `mn rd, 0(rd)`.
- Doc contract: base.rs:173 "Bare symbol: \"ld rd, symbol\" pseudo-instruction" — asserted fingerprint 5898478b
- Seed: encode_jal_pbt.rs reloc-form
- Formal: ∀ mn, rd, s. encode_load([Reg(rd), Symbol(s)], f3) = WordsWithRelocs[(auipc_word, PcrelHi20, s, 0), (load_word, PcrelLo12I, s, 0)] ∧ auipc_word = llvm-mc("auipc rd, 0") ∧ load_word = llvm-mc("mn rd, 0(rd)") ∧ Label(s) agrees
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, s]
  domain: { mn: load_mnemonics, rd: gpr, s: ident }
  body: WordsWithRelocs two-word auipc+load with PcrelHi20/PcrelLo12I on s
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  s: { gen: string, type: String }
evidence: base.rs:173-186; README.md:387 R_RISCV_PCREL_LO12_I/S; llvm-mc `ld rd, foo` expands to auipc+ld
```

## encode_load_neg_imm_oob
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: offset must be in [-2048, 2047]. Documented I-type imm[11:0] (README.md:353). llvm-mc rejects 2048 and -2049. No encode_load comment declares oob valid. Domain is the documented I-type range; oob must Err.
- Doc contract: README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" — asserted fingerprint a59ffa62
- Seed: encode_jalr_pbt.rs encode_jalr_neg_imm_oob
- Formal: ∀ mn, rd, rs1, off ∉ [-2048,2047]. llvm-mc rejects `mn rd, off(rs1)` ⇒ encode_load([Reg(rd), Mem{rs1,off}], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: failing
- Counterexample: encode_load([Reg("x0"), Mem{base:"x0", offset:2048}], 0) = Ok(Word(0x80000003))
- Bug report: pbt-out/bug_reports/encode_load_imm_oob.md

```property
function: encode_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, off]
  domain: { off: oob_i12 }
  relation:
    op: throws
    expr: encode_load(vec![Reg(rd), Mem{rs1, off}], funct3(mn))
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc operand must be an integer in the range [-2048, 2047]; README.md:353 imm[11:0]
```

## encode_load_neg_extra
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects a third operand (`ld x1, 0(x2), x3`). encode_instruction passes the full operand slice through. Extra operands must Err.
- Doc contract: base.rs:190 "load: expected memory operand" — asserted fingerprint 20f10956
- Seed: encode_jal_pbt.rs encode_jal_neg_extra
- Formal: ∀ mn, rd, rs1, off∈[-2048,2047], extra. encode_load([Reg(rd), Mem{rs1,off}, extra], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: failing
- Counterexample: encode_load([Reg("x0"), Mem{base:"x0", offset:0}, Imm(0)], 0) = Ok(Word(0x00000003))
- Bug report: pbt-out/bug_reports/encode_load_extra_operand.md

```property
function: encode_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, off, extra]
  domain: { extra: Operand }
  relation:
    op: throws
    expr: encode_load(vec![Reg(rd), Mem{rs1, off}, extra], funct3(mn))
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: llvm-mc invalid operand for instruction on extra operand; encode_instruction passes operands through
```

## encode_load_neg_arity_fp
- Tier: 3
- Rationale: Negative/error contract: empty list, missing 2nd operand, FP dest, FP/invalid base, Imm/Csr/Fence/RoundingMode as 2nd operand must Err (`load: expected memory operand` or invalid register).
- Doc contract: base.rs:190 "load: expected memory operand" — asserted fingerprint 20f10956
- Seed: encode_jalr_pbt.rs encode_jalr_neg_fp / encode_jalr_neg_empty
- Formal: ∀ bad ∈ {[], [Reg(rd)], [Reg(fp), Mem], [Reg(rd), Mem{fp_base}], [Reg(rd), Imm|Csr|Fence|RoundingMode]}. encode_load(bad, f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, bad]
  domain: { bad: invalid_load_operands }
  relation:
    op: throws
    expr: encode_load(bad, funct3(mn))
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  bad: { gen: string, type: VecOperand }
expected_error: String
evidence: base.rs:148 get_reg; base.rs:152 invalid base register; base.rs:190 load expected memory operand
```

## encode_load_neg_hi_modifier
- Tier: 3
- Rationale: Sweep — MemSymbol Hi20/PcrelHi20/TprelHi20 remap arms (base.rs:161-163) are documented as load-type Lo12 conversion, but llvm-mc rejects %hi/%pcrel_hi/%tprel_hi on loads (only %lo/%pcrel_lo/%tprel_lo). Negative/error contract from the reference assembler the SUT claims to replace.
- Doc contract: base.rs:157 "Use Lo12I for load-type relocations" — asserted fingerprint 1c7049c6
- Seed: encode_load_reloc_lo (valid lo modifiers); llvm-mc error text
- Formal: ∀ mn, rd, rs1, s, hi ∈ {%hi,%pcrel_hi,%tprel_hi}. llvm-mc rejects `mn rd, %hi(s)(rs1)` ⇒ encode_load([Reg(rd), MemSymbol{rs1, "%hi(s)"}], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: failing
- Counterexample: encode_load([Reg("x0"), MemSymbol{base:"x0", symbol:"%hi(foo)"}], 0) = Ok(WordWithReloc{word:3, Lo12I, "foo", 0})
- Bug report: pbt-out/bug_reports/encode_load_hi_modifier.md

```property
function: encode_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, s, hi]
  domain: { hi: hi_pcrel_hi_tprel_hi }
  relation:
    op: throws
    expr: encode_load(vec![Reg(rd), MemSymbol{rs1, hi_mod(s)}], funct3(mn))
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  s: { gen: string, type: String }
  hi: { gen: oneof, options: [hi, pcrel_hi, tprel_hi] }
expected_error: String
evidence: llvm-mc operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier; base.rs:161-163 Hi20 remap
```

## encode_load_symbol_offset_addend
- Tier: 5
- Rationale: Sweep — Operand::SymbolOffset is a documented parser production (parser.rs:29-30) and llvm-mc accepts `ld rd, foo+4` as the same auipc+ld expansion as a bare symbol, with the addend on %pcrel_hi. encode_load currently matches only Symbol/Label and falls through to Err. Same-job as the documented bare-symbol pseudo (base.rs:173-175) plus addend.
- Doc contract: base.rs:173 "Bare symbol: \"ld rd, symbol\" pseudo-instruction" — asserted fingerprint 5898478b
- Seed: encode_jal_pbt.rs encode_jal_symbol_offset_addend
- Formal: ∀ mn, rd, s, addend≠0. encode_load([Reg(rd), SymbolOffset(s, addend)], f3) = WordsWithRelocs[(auipc, PcrelHi20, s, addend), (load, PcrelLo12I, s, _)]
- Test file: src/backend/riscv/assembler/encoder/encode_load_pbt.rs
- Status: failing
- Counterexample: encode_load([Reg("x0"), SymbolOffset("foo", 1)], 0) = Err("load: expected memory operand")
- Bug report: pbt-out/bug_reports/encode_load_symbol_offset.md

```property
function: encode_load
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, s, addend]
  domain: { addend: nonzero_i64 }
  relation:
    op: holds
    expr: words_with_relocs_auipc_load_with_addend(encode_load(vec![Reg(rd), SymbolOffset(s, addend)], funct3(mn)), s, addend)
generators:
  mn: { gen: oneof, options: [lb, lh, lw, ld, lbu, lhu, lwu] }
  rd: { gen: string, type: String }
  s: { gen: string, type: String }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: llvm-mc ld x1, foo+4 expands to auipc %pcrel_hi(foo+4) plus ld; parser.rs:29 SymbolOffset; base.rs:173-175 bare-symbol expansion
```
