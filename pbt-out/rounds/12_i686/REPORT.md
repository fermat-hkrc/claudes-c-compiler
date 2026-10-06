# PBT Campaign Report: src/backend/i686 (round 12)

## Summary

**Verdict:** 1 medium bug to fix (#770): the i686 target strips the arch-neutral gcc-internal include path, so `ccc -m32` cannot compile `#include <stdio.h>` programs out of the box on standard multilib hosts (worked around in the harness with -I). With the workaround, the i686 pipeline is CORRECT: 160 generated programs (u64 expression functions on the 32-bit target — register-pair paths, 1..10-arg ILP32 stack args — plus a loop kernel and double functions) compiled at -O0 and -O2 execute natively with stdout identical to gcc -m32.
**Date:** 2026-10-05
**Modules tested:** src/backend/i686 (full pipeline composition)
**Tests:** 1 E2E property
**Result:** 1 passing (workaround documented), 1 bug (#770)
**Coverage evidence:** none (native; manual session). Behavioral: full-pipeline native execution vs gcc -m32.
**Tier:** standard

## Modules Tested
| Module | Tests | Bugs | Oracles |
|---------|-------|------|---------|
| i686 codegen+peephole+assembler+linker (composition) | 1 (P1) | 1 (#770, bring-up) | differential (native vs gcc -m32) |

## Bugs Found
### B1: i686 target strips the arch-neutral gcc-internal include path (#770)
**Formal:** target switch must preserve the ability to resolve freestanding headers. Falsified: ccc -m32 on `#include <stdio.h>` → "stddef.h: No such file or directory".
**Contract evidence:** inferred (freestanding headers are target-agnostic; the host gcc-internal dir is the only source on multilib hosts)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** ccc -m32 -o t t.c with t.c = `#include <stdio.h>\nint main(void){return 0;}`
**Expected / Actual:** compiles / 2 preprocessor errors
**Impact:** i686 target unusable out of the box on the standard gcc+multilib setup.
**Root cause:** predefined_macros.rs set_target i686 arm retain(!contains("x86_64")) removes /usr/lib/gcc/x86_64-linux-gnu/<v>/include; replacement cross dirs don't exist on multilib hosts.
**Bug report:** bug_reports/backend_i686_include_path_strips_gcc_internal.md (issue #770)
**Repro seed:** deterministic (see report)
**Raw output:** (in the bug report)

## Test Files Created
| File | Tests |
|------|-------|
| src/backend/i686/mod.rs (pbt_e2e) | p1_i686_e2e_matches_gcc_m32 |

## Reproduction
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::i686::pbt_e2e --nocapture

## Output Directories
- pbt-out/rounds/12_i686/ (PLAN, PROPERTIES, REPORT, report.json, bug_reports/, run/)
