# Properties: encode_fence

## encode_fence_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FENCE decoder). encode_i / fence.i / fence.tso rejected as same-job siblings (private packer / different mnemonics). Domain is llvm-mc-valid letter combinations: non-empty subsequences of iorw selected in order.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_csr_pbt.rs:368 encode_csr_diff_llvm_mc
- Formal: ∀ pred, succ ∈ in-order-subsequences(iorw). encode_fence([FenceArg(pred), FenceArg(succ)]) = Word(w) ∧ w = llvm-mc("fence pred, succ")
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fence
oracle: differential
predicate:
  quantifier: forall
  vars: [pred, succ]
  domain: { pred: in_order_iorw, succ: in_order_iorw }
  relation:
    op: eq
    lhs: sut_word([FenceArg(pred), FenceArg(succ)])
    rhs: llvm_mc_word("fence " + pred + ", " + succ)
generators:
  pred: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
  succ: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
evidence: encoder/mod.rs:682 fence => encode_fence; llvm-mc -triple=riscv64 -show-encoding
```

## encode_fence_empty_is_iorw
- Tier: 3
- Rationale: Algebraic metamorphic from system.rs:7 and llvm-mc (bare `fence` encodes identically to `fence iorw, iorw`). Stronger differential for the empty case is included as a KAT; this property checks the SUT identity independently of llvm-mc so a packer regression still fails.
- Doc contract: system.rs:7 "fence iorw, iorw" — asserted fingerprint 525bd367
- Seed: (none)
- Formal: ∀. encode_fence([]) = encode_fence([FenceArg("iorw"), FenceArg("iorw")]) = Word(0x0FF0000F)
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fence
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [unit]
  domain: { unit: unit }
  relation:
    op: eq
    lhs: sut_word([])
    rhs: sut_word([FenceArg("iorw"), FenceArg("iorw")])
generators:
  unit: { gen: const, value: 0 }
evidence: system.rs:7 empty operands comment fence iorw, iorw; llvm-mc fence encoding [0x0f,0x00,0xf0,0x0f]
```

## encode_fence_i_type_fields
- Tier: 4
- Rationale: Algebraic invariant from RISC-V ISA / README.md:353 / encoder/mod.rs:303 I-type layout. Weaker than differential (does not pin pred/succ bit assignment against an independent assembler) but catches rd/rs1/funct3/fm/opcode packing bugs even if llvm-mc is unavailable. Independent unpack, not a copy of encode_i.
- Doc contract: encoder/mod.rs:303 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: encode_csr_pbt.rs:399 encode_csr_i_type_fields
- Formal: ∀ pred, succ ∈ in-order-subsequences(iorw). let w = encode_fence([FenceArg(pred), FenceArg(succ)]) in opcode(w)=0b0001111 ∧ rd(w)=0 ∧ funct3(w)=0 ∧ rs1(w)=0 ∧ fm(w)=0 ∧ pred_bits(w)=iorw_mask(pred) ∧ succ_bits(w)=iorw_mask(succ)
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fence
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [pred, succ]
  domain: { pred: in_order_iorw, succ: in_order_iorw }
  relation:
    op: holds
    expr: unpack_fence_ok(sut_word([FenceArg(pred), FenceArg(succ)]), pred, succ)
generators:
  pred: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
  succ: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
evidence: README.md:353 I-type layout; encoder/mod.rs:303 I-type; encoder/mod.rs:357 OP_MISC_MEM; RISC-V ISA FENCE fm|pred|succ
```

## encode_fence_imm0_diff_llvm_mc
- Tier: 2
- Rationale: llvm-mc accepts numeric 0 as a fence operand ("or be 0"). The parser emits Operand::Imm(0) for `fence 0, ...` (is_fence_arg rejects '0'). Differential over Imm(0) mixed with in-order letter args. Stronger than a crash-only check because the encoding is specified.
- Doc contract: (none) on encode_fence for numeric 0. Contract inferred from llvm-mc operand rule and parser.rs:1004 is_fence_arg (digits are not FenceArg). fingerprint (none)
- Seed: encode_csr_pbt.rs:384 encode_csr_imm_auto_diff_llvm_mc
- Formal: ∀ a, b ∈ in-order-subsequences(iorw) ∪ {0}. encode_fence(op(a), op(b)) = llvm-mc("fence a, b") where op(0)=Imm(0) and op(letters)=FenceArg(letters)
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: failing
- Counterexample: encode_fence([Imm(0), Imm(0)])
- Bug report: pbt-out/bug_reports/encode_fence_imm0_as_full_barrier.md

```property
function: encoder.encode_fence
oracle: differential
predicate:
  quantifier: forall
  vars: [a, b]
  domain: { a: in_order_iorw_or_zero, b: in_order_iorw_or_zero }
  relation:
    op: eq
    lhs: sut_word([op_fence(a), op_fence(b)])
    rhs: llvm_mc_word("fence " + a + ", " + b)
generators:
  a: { gen: oneof, options: [0, i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
  b: { gen: oneof, options: [0, i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
evidence: llvm-mc "operand must be formed of letters selected in-order from 'iorw' or be 0"; parser.rs:1004 is_fence_arg
```

## encode_fence_neg_extra
- Tier: 5
- Rationale: llvm-mc rejects a third operand (`invalid operand for instruction`). encode_instruction passes operands through, so extra operands are caller-reachable. Negative/error contract: encode_fence must Err. Stronger differential does not apply on the error path (no encoding to compare).
- Doc contract: (none) on encode_fence for extra operands. Contract inferred (llvm-mc + public wrapper encode_instruction passes operands through). fingerprint (none)
- Seed: encode_csr_pbt.rs:474 encode_csr_neg_extra
- Formal: ∀ pred, succ ∈ in-order-subsequences(iorw), extra ∈ Operand. encode_fence([FenceArg(pred), FenceArg(succ), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: failing
- Counterexample: encode_fence([FenceArg("i"), FenceArg("i"), Imm(0)]) = Ok(Word)
- Bug report: pbt-out/bug_reports/encode_fence_extra_operand.md

```property
function: encoder.encode_fence
oracle: negative_error
predicate:
  quantifier: forall
  vars: [pred, succ, extra]
  domain: { pred: in_order_iorw, succ: in_order_iorw, extra: Operand }
  relation:
    op: throws
    expr: encode_fence([FenceArg(pred), FenceArg(succ), extra])
expected_error: String
generators:
  pred: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
  succ: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
  extra: { gen: oneof, options: [Imm(0), Imm(-1), Reg(x0), Symbol(foo), FenceArg(iorw)] }
evidence: llvm-mc fence iorw, iorw, x0 -> invalid operand; encoder/mod.rs:682 operands passed through
```

## encode_fence_neg_arity
- Tier: 5
- Rationale: llvm-mc rejects a single fence operand (`too few operands for instruction`). Empty is valid (defaults to iorw,iorw); one operand is not. Negative/error contract: encode_fence must Err for len==1.
- Doc contract: (none) on encode_fence for arity 1. Contract inferred (llvm-mc). fingerprint (none)
- Seed: encode_csr_pbt.rs:544 encode_csr_neg_arity_fp_unknown
- Formal: ∀ op ∈ Operand. encode_fence([op]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: failing
- Counterexample: encode_fence([FenceArg("iorw")]) = Ok(Word(0x0ff0000f))
- Bug report: pbt-out/bug_reports/encode_fence_arity_one.md

```property
function: encoder.encode_fence
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: Operand }
  relation:
    op: throws
    expr: encode_fence([op])
expected_error: String
generators:
  op: { gen: oneof, options: [FenceArg(iorw), FenceArg(rw), Imm(0), Reg(x0), Symbol(foo)] }
evidence: llvm-mc fence iorw -> too few operands; llvm-mc fence 0 -> too few operands
```

## encode_fence_neg_out_of_order
- Tier: 5
- Rationale: llvm-mc requires letters selected in-order from iorw (rejects wroi, irow, ii, IORW). parse_fence_bits uses contains() so order/duplicates/case are ignored. Negative/error: encode_fence must Err (or at least not silently encode a valid-looking fence) for out-of-order, duplicate, and uppercase letter strings that llvm-mc rejects.
- Doc contract: encoder/mod.rs:442 "Parse a fence ordering string (e.g., \"iorw\") into a 4-bit mask." — asserted fingerprint 7cb05145. The helper comment does not declare out-of-order input invalid; llvm-mc does. Classification: asserted (mask from iorw letters) not a domain restriction excluding order.
- Seed: (none)
- Formal: ∀ pred ∈ {wroi, irow, ri, wi, oi, wr, ro, wo, ii, rr, ww, IORW, I, Rw}, succ ∈ in-order-subsequences(iorw). encode_fence([FenceArg(pred), FenceArg(succ)]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: failing
- Counterexample: encode_fence([FenceArg("wroi"), FenceArg("i")]) = Ok(Word)
- Bug report: pbt-out/bug_reports/encode_fence_out_of_order_letters.md

```property
function: encoder.encode_fence
oracle: negative_error
predicate:
  quantifier: forall
  vars: [pred, succ]
  domain: { pred: invalid_fence_letters, succ: in_order_iorw }
  relation:
    op: throws
    expr: encode_fence([FenceArg(pred), FenceArg(succ)])
expected_error: String
generators:
  pred: { gen: oneof, options: [wroi, irow, ri, wi, oi, wr, ro, wo, ii, rr, ww, IORW, I, Rw] }
  succ: { gen: oneof, options: [i, o, r, w, io, ir, iw, or, ow, rw, ior, iow, irw, orw, iorw] }
evidence: llvm-mc operand must be formed of letters selected in-order from iorw or be 0
```

## encode_fence_neg_invalid_operand
- Tier: 5
- Rationale: llvm-mc rejects registers, symbols, and numeric values other than 0 as fence operands. encode_fence currently maps any non-FenceArg to 0xF. Negative/error: those operands must Err.
- Doc contract: (none) on encode_fence for operand kinds. Contract inferred (llvm-mc). fingerprint (none)
- Seed: encode_csr_pbt.rs:544 encode_csr_neg_arity_fp_unknown
- Formal: ∀ kind ∈ {Reg, Symbol, Csr, RoundingMode, Mem, Imm(n) where n≠0}. encode_fence([kind, FenceArg("rw")]) is Err ∧ encode_fence([FenceArg("rw"), kind]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
- Status: failing
- Counterexample: encode_fence([Reg("x0"), FenceArg("rw")]) = Ok(Word)
- Bug report: pbt-out/bug_reports/encode_fence_invalid_operand.md

```property
function: encoder.encode_fence
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: non_fence_operand }
  relation:
    op: throws
    expr: encode_fence([bad, FenceArg("rw")])
expected_error: String
generators:
  bad: { gen: oneof, options: [Reg(x0), Symbol(foo), Csr(mstatus), RoundingMode(rne), Imm(1), Imm(15), Imm(-1), Imm(16)] }
evidence: llvm-mc fence x0, x0 / fence 1, 2 / fence 15, 15 -> operand must be letters in-order from iorw or be 0
```
