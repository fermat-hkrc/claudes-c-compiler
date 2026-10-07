# Confirmed invariants (encode_bgt)

## Behavioral invariants
- `bgt rs, rt, tgt` encodes as B-type BLT with registers swapped: B-type rs1=rt, rs2=rs, funct3=0b100, opcode=0b1100011, imm field 0; reloc = Branch(symbol=tgt, addend=0).
- ABI names, xN, fp/s0/x8, and zero/x0 aliases produce the same encoding for the same register numbers.
- Imm(0..31) as either register operand (GCC bare-number path) equals the corresponding xN form.
- Symbol / Label / Reg-as-label targets with the same string are equivalent in the reloc symbol.
- Imm targets are stringified into reloc.symbol; the machine-word immediate stays 0.
- Under-arity (<3 operands), invalid GPRs, and invalid target operand kinds return Err.
- Word agrees with llvm-mc `bgt rs, rt, 0` and with llvm-mc `blt rt, rs, 0` for all sampled GPR name pairs.
- Metamorphic: encode_bgt([rs,rt,tgt]) equals encode_branch_instr([rt,rs,tgt], funct3=BLT).

## Environment (encode_bgt)
- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc (LLVM 15.0.6), `-triple=riscv64 -show-encoding`.
- Harness: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs, cargo test --lib encode_bgt_, proptest cases=1000.
- Dispatch: encoder/mod.rs:902 "bgt" => encode_bgt(operands).
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and said NOT LINKED).

## Quirks (encode_bgt)
- encode_bgt has no rustdoc; contract is README.md:330 plus the inline `// blt rs2, rs1` comment.
- **Bug:** trailing operands beyond index 2 are silently ignored (same family defect as sibling branch pseudos). Property encode_bgt_neg_extra fails; see bug_reports/encode_bgt_extra_operand.md.
- When rt=x0, llvm-mc pretty-prints `bgtz rs`; when rs=x0, it pretty-prints `bltz rt`; encodings still match.
