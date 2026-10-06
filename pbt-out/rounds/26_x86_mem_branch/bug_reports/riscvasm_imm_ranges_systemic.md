# RISC-V assembler silently accepts out-of-range immediates across 4 more instruction paths (auipc negative, jalr ±2048 boundary, csrrw csr ≥ 4096, sb 2048) — #796's class is systemic
> **Filed as [issue #824](https://github.com/fermat-hkrc/claudes-c-compiler/issues/824)**

# Bug: RISC-V assembler silently accepts out-of-range immediates across 4 more instruction paths

**Law:** Each RISC-V instruction's immediate field has a fixed width and range; an out-of-range immediate is unencodable and must be rejected (the ISA field width is the contract). Round 17 (#796) filed the imm12 range on the addi/load/store path; this round's boundary battery shows the SAME missing-range-check on four MORE paths:

| Form | Valid range | ccc bytes (decodes to) |
|------|-------------|------------------------|
| `auipc a0, -1` | [0, 65535] (U-type imm20 **unsigned**) | `17f5ffff` — imm20 = 0xFFFFF (≈ 4MB offset, not -4KB) |
| `jalr a0, 2048(a1)` | [-2048, 2047] (I-type imm12) | `67850580` — imm12 bits = 0x800 (decodes as **-2048**) |
| `jalr a0, -2049(a3)` | [-2048, 2047] | silently accepted (low 12 bits = 2047) |
| `csrrw a0, 4096, a1` | [0, 4095] (12-bit CSR index) | `73950500` — CSR field = 0 (**cycle** reg, not 4096) |
| `sb a0, 2048(a1)` | [-2048, 2047] (S-type imm12) | `2380a580` — decodes as sb #2047 |

**Impact:** External `.s` files with boundary typos silently assemble to different instructions than written — e.g. `csrrw a0, 4096, a1` accesses CSR 0 (cycle) instead of erroring; `jalr a0, 2048(a1)` jumps to -2048.

**Function:** RISC-V assembler immediate encoding (U/I/S-type paths + CSR index)

**Detected by:** PBT round-26g RISC-V boundary battery (16 forms; the 5 silent-accepts above; every in-range form byte-identical to llvm-mc).

**Minimal input:**
```asm
.text
sb a0, 2048(a1)
```
```bash
ccc-riscv -c t.s -o t.o   # assembles!
llvm-mc -triple=riscv64 -filetype=obj t.s   # error: immediate must be an integer in range [-2048, 2047]
```

**Severity:** medium (silent wrong instruction — #796's class confirmed systemic across paths)

**Contract evidence:** inferred (ISA field widths; llvm-mc rejects every form; the encodings provably change semantics)

**Fix:** range-check every immediate slot per instruction format: U-type imm20 [0,65535] for lui/auipc (negative values require manual 2^20 arithmetic), I-type imm12 for jalr, S-type imm12 for stores, 12-bit unsigned CSR index for csrr* — mirror the imm12 check #796 already needs.

**Regression test:** deterministic: each of the five forms must be rejected.
