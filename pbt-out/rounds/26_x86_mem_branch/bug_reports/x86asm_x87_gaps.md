# x87 assembler subset gaps: fst %st(i), fcomi, the whole fcmovcc family, fstsw/fsave/frstor aliases, ffree/fnop/fincstp/fdecstp
> **Filed as [issue #819](https://github.com/fermat-hkrc/claudes-c-compiler/issues/819)**

**Law:** the assembler's documented x87 FPU coverage includes the standard instruction set; both GNU as and llvm-mc assemble every form below.

**Impact:** external .s files (hand-written math kernels, long-double code paths) fail to assemble. All double-reference confirmed (as AND llvm-mc accept, ccc rejects):
- `fst %st(i)` — non-popping store to a stack register (fstp works)
- `fcomi %st(i), %st` — ordered compare-and-set-flags (fcomip AND fucomi/fucomip work; only the base fcomi form is missing)
- `fcmovcc %st(i), %st` — the ENTIRE conditional-move family (b/e/be/u/nb/ne/nbe/nu — 8 conditions)
- `fstsw %ax` / `fstsw mem` — store-status-word alias (fnstsw works)
- `fsave`/`frstor` (fnsave/fnstenv-family aliases)
- `fincstp`/`fdecstp`/`ffree %st(i)`/`fnop`

**Function:** x86 assembler x87 encoder
**Source location:** src/backend/x86/assembler/encoder/x87_misc.rs (missing arms / no-wait-alias table incomplete)
**Detected by:** round-26 x87 battery (51 forms; 28+12 clean, 16 GAPs all in the listed families)
**Minimal input:** `.text\nfstsw %ax` (the classic fcom+fstsw+sahf compare idiom) → ccc rejects
**Expected:** assembles
**Actual:** rejected
**Severity:** low (completeness; same class as #777/#782/#818)
**Contract evidence:** inferred (double-reference acceptance; x86/README documents x87 FPU support)
**Fix:** add the missing arms: fst st(i) (D8 D0+i / DD D8+i), fcomi (DB F0+i), fcmovcc (DA C0+i / DB C0+i per condition), the wait-alias forms (fstsw=fnsaw alias 9B DF E0; fsave/fnsave 9B DD / DD), ffree (DD C0+i), fnop (D9 D0), fincstp/fdecstp (D9 F7 / F6).
**Regression test:** deterministic: assemble the six minimal forms.
