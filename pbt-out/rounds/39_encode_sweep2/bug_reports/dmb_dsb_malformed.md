# encode_dmb/encode_dsb accept EMPTY and WRONG-TYPE operands (dmb x0, bare dsb/dmb, dmb #ish) — silently encode sy; llvm rejects all (different axes than #728/#732 extra-operand)
> **Filed as [issue #1131](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1131)**

# Bug: encode_dmb/encode_dsb accept EMPTY and WRONG-TYPE operands (dmb x0, bare dsb/dmb, dmb #ish) — silently encode sy; llvm rejects all (different axes than #728/#732 extra-operand)
**Law:** ∀ barrier mnemonic M ∈ {dmb, dsb}: M requires exactly one valid barrier-option operand; empty operand lists or non-barrier operand kinds (Reg/Imm-prefixed-name) must return Err. FALSIFIED — all silently encode `M sy`.
**Impact:** Every malformed barrier form assembles as a FULL-SYSTEM barrier with no diagnostic:
```
form         llvm-mc                            ccc (objdump)
dmb x0       error: invalid barrier option name  dmb sy
dsb x0       error: invalid barrier option name  (same path)
dsb (empty)  error: too few operands             dsb sy
dmb (empty)  error: too few operands             dmb sy
dmb #ish     error: immediate value expected     dmb sy  (hash-prefixed name silently taken)
```
A register/typo'd option becomes the STRONGEST barrier — semantically over-synchronizing instead of erroring (silent behavioral change, not just a dropped token).
**Function:** encode_dmb (src/backend/arm/assembler/encoder/system.rs:6), encode_dsb (:30) — the operand-kind match defaults to sy for anything unrecognized/absent.
**Separate from #728/#732:** those cover EXTRA operands (2-valid + 1-extra); this is the missing-operand and wrong-operand-KIND axes — the count-check fix for #728/#732 would not fix these (kind check needed).
**Detected by:** Negative/Error Contract — round-39 malformed matrix `p_malformed_system` (catch_unwind sweep: silent-Ok on &[Reg] and &[]), llvm-mc ground truth above
**Minimal input:** `encode_dsb(&[])` or assembly `dsb` — Ok(dsb sy) instead of Err.
**Expected:** Err for all four forms (llvm messages quoted above).
**Actual:** Ok — `d5033f9f` (dsb sy) / `d5033fbf` (dmb sy) emitted.
**Fix sketch:** `match operands { [Barrier(opt)] => ..., _ => return Err("barrier option required") }` — one kind+arity match per function covers all axes including #728/#732.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nt:\n    dmb x0\n    dsb\n    dmb\n' > /tmp/d.s
./target/debug/ccc-arm /tmp/d.s -c -o /tmp/d.o && aarch64-linux-gnu-objdump -d /tmp/d.o | grep -E 'dmb|dsb'
# llvm: echo 'dsb' | llvm-mc --triple=aarch64 --show-encoding  → error: too few operands
```
