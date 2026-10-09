# Coverage Status — encode_double_shift campaign

**Date:** 2026-10-09
**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (build tree not reconfigured for instrumentation). Tooling also reported `encode_double_shift` NOT LINKED (mangling/inlining false negative); the live suite exercises it via `InstructionEncoder::encode` → dispatch → `encode_double_shift` (KAT + 1000-case properties produced concrete byte vectors and four SUT bugs).

## This campaign

| Function | Indexed | Properties | Executed (behavioral) | Notes |
|----------|---------|------------|----------------------|-------|
| encode_double_shift | yes (gp_integer.rs:920) | 10 | yes | Imm/CL RR, arity reject, bad-count reject exercised; mem arms absent (bug); Imm8 overflow truncated (bug); non-GP/width accepted (bugs) |

## Sweep round 1 (standard)

- Called `coverage_gaps` after first full test run.
- Documented behaviors already targeted: Imm RR, CL RR, mem dst (Intel r/m32), Imm8 domain, GP class, width match, arity, count-reg class, mnemonic alias.
- No additional documented branch without a property: missing mem arms are missing code (filed as B1), not unexecuted dead code. `_size` unused — no 16-bit dispatch path exists at call sites (only size=4).
- Sweep closed: every documented contract surface for this sole target has a property.

## Prior campaigns

See historical rows in COVERAGE.md (many modules across ARM/RISC-V/i686).
