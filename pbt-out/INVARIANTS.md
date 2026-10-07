# Confirmed invariants (encode_ble)

## Behavioral invariants
- `ble rs, rt, tgt` encodes as B-type BGE with registers swapped: B-type rs1=rt, rs2=rs, funct3=0b101, opcode=0b1100011, imm field 0; reloc = Branch(symbol=tgt, addend=0).
- ABI names, xN, fp/s0/x8, and zero/x0 aliases produce the same encoding for the same register numbers.
- Imm(0..31) as either register operand (GCC bare-number path) equals the corresponding xN form.
- Symbol / Label / Reg-as-label targets with the same string are equivalent in the reloc symbol.
- Imm targets are stringified into reloc.symbol; the machine-word immediate stays 0.
- Under-arity (<3 operands), invalid GPRs, and invalid target operand kinds return Err.
- Word agrees with llvm-mc `ble rs, rt, 0` and with llvm-mc `bge rt, rs, 0` for all sampled GPR name pairs.
- Metamorphic: encode_ble([rs,rt,tgt]) equals encode_branch_instr([rt,rs,tgt], funct3=BGE).

## Environment (encode_ble)
- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc (LLVM 15.0.6), `-triple=riscv64 -show-encoding`.
- Harness: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs, cargo test --lib encode_ble_, proptest cases=1000.
- Dispatch: encoder/mod.rs:905 "ble" => encode_ble(operands).
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and said NOT LINKED).

## Quirks (encode_ble)
- encode_ble has no rustdoc; contract is README.md:330 plus the inline `// bge rs2, rs1` comment.
- **Bug:** trailing operands beyond index 2 are silently ignored (same family defect as sibling branch pseudos encode_bgt/blez/…). Property encode_ble_neg_extra fails; see bug_reports/encode_ble_extra_operand.md.
- When rt=x0, llvm-mc pretty-prints `blez rs`; when rs=x0, it pretty-prints `bgez rt`; encodings still match.
