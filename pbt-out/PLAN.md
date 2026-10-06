# PBT Campaign: src/driver + glue (round 14 — final round of the whole-repo loop)

> Rounds 01–13 done. This round covers the driver layer (CLI parsing, mode
> dispatch, external-tool fallback wiring) and closes the loop over src/.

## Scan findings

- **Spec:** src/driver/ (cli.rs flag table, pipeline.rs mode dispatch, external_tools.rs fallbacks); main.rs/lib.rs/compiler_main are 3–45 lines of glue.
- **Test layout:** inline #[cfg(test)]; the driver is exercised by every E2E round (09/12) and every -c assembly (10/11/13).
- **Candidate modules:** mode plumbing (-E/-S/-c/Full) × target selection (argv0 + -m32/-mlittle-endian): observable artifacts must exist and be well-formed per mode (preprocessed text / .s containing the function name / ET_REL object / runnable executable).
- **Skipped modules:** external_tools.rs GCC-fallback paths (require an intentionally-broken builtin to trigger — environment-fabrication out of scope; the builtin paths are covered by every other round); bin/*.rs and main.rs (3-line mains calling compiler_main); lib.rs (thread-spawn wrapper).

## Module: driver

- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
