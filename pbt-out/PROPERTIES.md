# Properties: encode_auipc

## encode_auipc_diff_imm_llvm_mc
- Tier: 4
- Rationale: Strongest applicable oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree AUIPC decoder). encode_lui / encode_c_lui / encode_u rejected (different opcode / compressed / shared packer).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_lui_pbt.rs encode_lui_diff_imm_llvm_mc
- Formal: ∀ rd ∈ {x0..x31 ∪ ABI names}, ∀ imm ∈ [0, 1048575]. encode_auipc([Reg(rd), Imm(imm)]) = Ok(Word(w)) ∧ w = llvm-mc("auipc rd, imm")
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_name, imm: int(0..1048575) }
  relation:
    op: eq
    lhs: encode_auipc([Reg(rd), Imm(imm)])
    rhs: llvm_mc_word("auipc {rd}, {imm}")
generators:
  rd: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: encoder/mod.rs:3 README.md:304 llvm-mc -triple=riscv64
```

## encode_auipc_isa_u_type
- Tier: 3
- Rationale: Algebraic invariant from README.md:356 / RISC-V U-type layout, independent of encode_u. Stronger differential is the sibling property; this pins opcode/rd/imm20 even if llvm-mc is unavailable.
- Doc contract: README.md:356 "U-type:  [          imm[31:12]           |  rd  | opcode]" — asserted fingerprint 8c9098fd
- Seed: encode_lui_pbt.rs encode_lui_isa_u_type
- Formal: ∀ rd ∈ 0..31, ∀ imm ∈ [0, 1048575]. let w = encode_auipc([Reg(x{rd}), Imm(imm)]). w[6:0]=0b0010111 ∧ w[11:7]=rd ∧ w[31:12]=imm
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: u32(0..31), imm: int(0..1048575) }
  body: unpack_u(encode_auipc([Reg(x{rd}), Imm(imm)])) == (0b0010111, rd, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: README.md:356 encoder/mod.rs:307 encoder/mod.rs:324
```

## encode_auipc_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names, xN, zero/ra/sp/fp aliases encode the same rd field. Independent of llvm-mc. Stronger differential covers xN vs llvm-mc; this checks SUT alias table.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_lui_pbt.rs encode_lui_abi_xn_alias
- Formal: ∀ n ∈ 0..31, ∀ imm ∈ [0, 1048575]. encode_auipc([Reg(x{n}), Imm(imm)]) = encode_auipc([Reg(ABI[n]), Imm(imm)]) ∧ (n=8 ⇒ encode_auipc([Reg("fp"), Imm(imm)]) equals both)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: u32(0..31), imm: int(0..1048575) }
  relation:
    op: eq
    lhs: encode_auipc([Reg(x{n}), Imm(imm)])
    rhs: encode_auipc([Reg(ABI[n]), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: encoder/mod.rs:146-191 parser.rs:22
```

## encode_auipc_reloc_pcrel_hi
- Tier: 4
- Rationale: Differential + reloc contract. `%pcrel_hi(symbol)` is the documented AUIPC reloc (encoder/mod.rs:57, README.md:334). llvm-mc emits R_RISCV_PCREL_HI20; reloc-form word equals `auipc rd, 0`.
- Doc contract: encoder/mod.rs:57 "R_RISCV_PCREL_HI20 - for AUIPC (high 20 bits of PC-relative)" — asserted fingerprint 9006ffa0
- Seed: encode_lui_pbt.rs encode_lui_reloc_hi
- Formal: ∀ rd ∈ GPR names, ∀ sym ∈ identifier. encode_auipc([Reg(rd), Symbol("%pcrel_hi("+sym+")")]) = Ok(WordWithReloc{word, PcrelHi20, symbol=sym, addend=0}) ∧ word = llvm-mc("auipc rd, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym]
  domain: { rd: gpr_name, sym: identifier }
  body: encode_auipc([Reg(rd), Symbol("%pcrel_hi("+sym+")")]) == WordWithReloc{llvm_mc("auipc rd, 0"), PcrelHi20, sym, 0}
generators:
  rd: { gen: string }
  sym: { gen: string }
evidence: encoder/mod.rs:57 README.md:334 llvm-mc -triple=riscv64
```

## encode_auipc_reloc_got_tls_hi
- Tier: 4
- Rationale: Differential over the remaining llvm-mc-accepted AUIPC modifiers (`%got_pcrel_hi`, `%tls_ie_pcrel_hi`, `%tls_gd_pcrel_hi`) mapping to GotHi20 / TlsGotHi20 / TlsGdHi20. encode_lui sibling rejected (LUI does not own GOT/TLS-GD AUIPC relocs).
- Doc contract: encoder/mod.rs:77 "R_RISCV_GOT_HI20 - GOT-relative AUIPC" — asserted fingerprint 2faffddb
- Seed: encode_lui_pbt.rs encode_lui_reloc_tprel_hi
- Formal: ∀ rd ∈ GPR names, ∀ sym ∈ identifier, ∀ (mod, kind) ∈ {(%got_pcrel_hi, GotHi20), (%tls_ie_pcrel_hi, TlsGotHi20), (%tls_gd_pcrel_hi, TlsGdHi20)}. encode_auipc([Reg(rd), Symbol(mod+"("+sym+")")]) = Ok(WordWithReloc{word, kind, symbol=sym, addend=0}) ∧ word = llvm-mc("auipc rd, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym, modifier]
  domain: { rd: gpr_name, sym: identifier, modifier: {got_pcrel_hi, tls_ie_pcrel_hi, tls_gd_pcrel_hi} }
  body: encode_auipc([Reg(rd), Symbol("%"+modifier+"("+sym+")")]) matches WordWithReloc{llvm_mc("auipc rd, 0"), mapped_kind(modifier), sym, 0}
generators:
  rd: { gen: string }
  sym: { gen: string }
  modifier: { gen: string }
evidence: encoder/mod.rs:77 encoder/mod.rs:79-82 llvm-mc -triple=riscv64
```

## encode_auipc_neg_imm_oob
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: AUIPC immediate must be in [0, 1048575]. SUT currently truncates via `(*imm as u32) << 12` with no range check. Stronger differential does not apply on the invalid domain.
- Doc contract: base.rs:47 "auipc: invalid operands" — asserted fingerprint 3ba99706
- Seed: encode_lui_pbt.rs encode_lui_neg_imm_oob
- Formal: ∀ rd ∈ GPR names, ∀ imm ∉ [0, 1048575]. llvm-mc rejects "auipc rd, imm" ⇒ encode_auipc([Reg(rd), Imm(imm)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: failing
- Counterexample: encode_auipc([Reg("x0"), Imm(-1)]) = Ok(Word(0xFFFFF017))
- Bug report: bug_reports/encode_auipc_imm_oob.md

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_name, imm: oob_imm }
  relation:
    op: throws
    expr: encode_auipc([Reg(rd), Imm(imm)])
generators:
  rd: { gen: string }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc AUIPC range [0, 1048575]; base.rs:47
```

## encode_auipc_neg_extra
- Tier: 3
- Rationale: Negative/error contract: AUIPC is two-operand; llvm-mc errors on a third operand. SUT matches only operands[0]/[1] and ignores extras.
- Doc contract: base.rs:47 "auipc: invalid operands" — asserted fingerprint 3ba99706
- Seed: encode_lui_pbt.rs encode_lui_neg_extra
- Formal: ∀ rd ∈ GPR names, ∀ imm ∈ [0, 1048575], ∀ extra. encode_auipc([Reg(rd), Imm(imm), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: failing
- Counterexample: encode_auipc([Reg("x0"), Imm(0), Imm(0)]) = Ok(Word(0x00000017))
- Bug report: bug_reports/encode_auipc_extra_operand.md

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, extra]
  domain: { rd: gpr_name, imm: int(0..1048575), extra: Operand }
  relation:
    op: throws
    expr: encode_auipc([Reg(rd), Imm(imm), extra])
generators:
  rd: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc invalid operand for instruction; base.rs:47
```

## encode_auipc_neg_bad_modifier
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: AUIPC symbol operands must use `%pcrel_hi`/`%got_pcrel_hi`/`%tls_ie_pcrel_hi`/`%tls_gd_pcrel_hi`. Plain symbols, `%hi`, `%lo`, `%pcrel_lo`, `%tprel_*` are rejected by llvm-mc. parse_reloc_modifier is a neighbouring helper, not encode_auipc's own domain restriction.
- Doc contract: encoder/mod.rs:57 "R_RISCV_PCREL_HI20 - for AUIPC (high 20 bits of PC-relative)" — asserted fingerprint 9006ffa0
- Seed: encode_lui_pbt.rs encode_lui_neg_bad_modifier
- Formal: ∀ rd ∈ GPR names, ∀ s ∈ {plain ident, %hi(ident), %lo(ident), %pcrel_lo(ident), %tprel_hi(ident), %tprel_lo(ident), %tprel_add(ident)}. llvm-mc rejects "auipc rd, s" ⇒ encode_auipc([Reg(rd), Symbol(s)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: failing
- Counterexample: encode_auipc([Reg("x0"), Symbol("foo")]) = Ok(WordWithReloc{PcrelHi20, "foo", 0})
- Bug report: bug_reports/encode_auipc_bad_modifier.md

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, s]
  domain: { rd: gpr_name, s: bad_auipc_symbol }
  relation:
    op: throws
    expr: encode_auipc([Reg(rd), Symbol(s)])
generators:
  rd: { gen: string }
  s: { gen: string }
expected_error: String
evidence: llvm-mc AUIPC modifier set; encoder/mod.rs:57
```

## encode_auipc_neg_arity
- Tier: 3
- Rationale: Negative/error contract: AUIPC requires two operands. llvm-mc reports too few operands. Strengthening round after the first batch.
- Doc contract: base.rs:47 "auipc: invalid operands" — asserted fingerprint 3ba99706
- Seed: encode_lui_pbt.rs encode_lui_neg_arity
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_auipc(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: throws
    expr: encode_auipc(ops)
generators:
  ops: { gen: string }
expected_error: String
evidence: llvm-mc too few operands; base.rs:47
```

## encode_auipc_neg_fp
- Tier: 3
- Rationale: Negative/error contract: AUIPC dest must be a GPR. llvm-mc rejects fa0/fN. Strengthening round.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_lui_pbt.rs encode_lui_neg_fp
- Formal: ∀ fp ∈ FP names, ∀ imm ∈ [0, 1048575]. encode_auipc([Reg(fp), Imm(imm)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, imm]
  domain: { fp: fp_name, imm: int(0..1048575) }
  relation:
    op: throws
    expr: encode_auipc([Reg(fp), Imm(imm)])
generators:
  fp: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
expected_error: String
evidence: llvm-mc invalid operand; parser.rs:22
```

## encode_auipc_neg_bad_operand
- Tier: 3
- Rationale: Negative/error contract: operand 1 must be Imm or Symbol. Labels, Mem, CSR, FenceArg, RoundingMode, SymbolOffset, MemSymbol are invalid. Strengthening round.
- Doc contract: base.rs:47 "auipc: invalid operands" — asserted fingerprint 3ba99706
- Seed: encode_lui_pbt.rs encode_lui_neg_bad_operand
- Formal: ∀ rd ∈ GPR names, ∀ op ∉ {Imm, Symbol}. encode_auipc([Reg(rd), op]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_auipc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, bad]
  domain: { rd: gpr_name, bad: non_imm_non_symbol }
  relation:
    op: throws
    expr: encode_auipc([Reg(rd), bad])
generators:
  rd: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: base.rs:47
```

## encode_auipc_pcrel_hi_addend
- Tier: 4
- Rationale: Differential/reloc contract: `%pcrel_hi(sym+N)` must relocate against `sym` with addend N (Relocation.addend, llvm-mc fixup value %pcrel_hi(foo+4)). Strengthening round; first batch did not cover addends.
- Doc contract: encoder/mod.rs:57 "R_RISCV_PCREL_HI20 - for AUIPC (high 20 bits of PC-relative)" — asserted fingerprint 9006ffa0
- Seed: encode_lui_pbt.rs encode_lui_hi_addend
- Formal: ∀ rd ∈ GPR names, ∀ sym ∈ identifier, ∀ a ≠ 0. encode_auipc([Reg(rd), Symbol("%pcrel_hi("+sym+±a+")")]) = Ok(WordWithReloc{PcrelHi20, symbol=sym, addend=a})
- Test file: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
- Status: failing
- Counterexample: encode_auipc([Reg("x0"), Symbol("%pcrel_hi(foo+1)")]) reloc.symbol = "foo+1", addend = 0
- Bug report: bug_reports/encode_auipc_pcrel_hi_addend.md

```property
function: encoder.encode_auipc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym, addend]
  domain: { rd: gpr_name, sym: identifier, addend: signed_addend }
  body: encode_auipc([Reg(rd), Symbol("%pcrel_hi("+sym+signed(addend)+")")]).reloc == {PcrelHi20, sym, addend}
generators:
  rd: { gen: string }
  sym: { gen: string }
  addend: { gen: int, type: i64 }
evidence: encoder/mod.rs:57 encoder/mod.rs:141-144 llvm-mc %pcrel_hi(foo+4)
```
