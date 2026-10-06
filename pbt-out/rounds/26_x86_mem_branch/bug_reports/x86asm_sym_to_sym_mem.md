# x86-64 assembler rejects symbol immediates to symbol memory destinations (addq $mydata, mydata2 etc.) — 5 forms missing
> **Filed as [issue #835](https://github.com/fermat-hkrc/claudes-c-compiler/issues/835)**

# Bug: x86-64 assembler rejects symbol immediates to symbol memory destinations
**Law:** An immediate that is a symbol address (`$mydata`) applied to a bare symbol memory operand (`mydata2` = absolute memory) is encodable via a 32-bit relocation (R_X86_64_32S) in the imm32 field. GNU as and llvm-mc assemble these.
**Impact:** External `.s` files using symbol-to-symbol arithmetic (`addq $mydata, mydata2` — adding a variable's address to another variable) fail to assemble. 5 affected forms: add/sub/or/and/mov with `$sym` to `sym2`. The corresponding `(%rax)` destination forms work fine.
**Function:** x86 assembler immediate-to-memory encoding with symbol relocation
**Detected by:** round-26r RIP-relative + symbol addressing battery (14 forms; 13 work; the 5 symbol-to-symbol GAPs)
**Minimal input:** `.text` / `.globl mydata` / `mydata: .quad 0` / `.globl mydata2` / `mydata2: .quad 0` / `addq $mydata, mydata2` → ccc: error; as: assembles
**Expected:** assembles (imm32 + R_X86_64_32S relocation to mydata)
**Actual:** rejected
**Severity:** low (rare pattern; but GNU as supports it)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\n.globl mydata\nmydata: .quad 0\n.globl mydata2\nmydata2: .quad 0\naddq $mydata, mydata2\n' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error: unsupported addq operands
as /tmp/t.s -o /dev/null  # OK
```

