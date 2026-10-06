# Bug: x86 tail-call optimization misses the common `return f(...)` shape whose result joins a shared epilogue label (deep self-recursion overflows at -O2)
> **Filed as [issue #805](https://github.com/fermat-hkrc/claudes-c-compiler/issues/805)**

**Law:** the peephole documents tail call optimization ("Phase 5: tail call optimization (call+epilogue+ret -> epilogue+jmp)"); the canonical tail-call shape — `return f(args)` compiled as call + jmp-to-shared-epilogue — must be converted to a jmp, as gcc does.

**Impact:** deep tail recursion (1,000,000 frames) runs fine under gcc -O2 (converted to a loop/jmp) but SEGFAULTS under ccc -O2 (plain call kept): `long sumto(long n, long acc) { return n == 0 ? acc : sumto(n-1, acc+n); }` → gcc prints 500000500000; ccc -O2 exits 139. Program-visible behavioral difference on legal C.

**Function:** peephole passes/tail_call.rs — is_tail_call_candidate
**Source location:** src/backend/x86/codegen/peephole/passes/tail_call.rs:143-218 — the scan requires the epilogue (movq %rbp,%rsp + popq %rbp + ret) INLINE within 30 lines after the call; any intervening LineKind::Jump/Label (the shared-epilogue join emitted for the OTHER return path) returns None.
**Detected by:** round-21 exotic-C battery (tail-recursion probe) vs gcc -O2, asm-confirmed (ccc -O2 sumto: 48-byte frame + 5 callee-save pushes + `call sumto`; gcc -O2: frame-free loop/jmp)
**Minimal input:** the sumto above with main calling sumto(1000000, 0)
**Expected:** ccc -O2 converts the tail call (or at minimum survives 1M frames like gcc)
**Actual:** call kept → stack overflow (exit 139)
**Severity:** low (quality/stack-depth; no wrong result — pure crash at extreme depth) — filed because the feature is documented
**Contract evidence:** documented — src/backend/x86/codegen/peephole/README.md "| passes/tail_call.rs | ... Phase 5: tail call optimization (call+epilogue+ret -> epilogue+jmp) |"
**Fix:** in is_tail_call_candidate, follow a single unconditional jump to a local label (bounded, e.g. one hop) and validate the epilogue there; or make codegen emit the tail call's result path inline.
**Regression test:** (deterministic: compile sumto at -O2, disassemble, assert no `call sumto` inside sumto — or run depth 1M expecting exit 0)
**Related note (not filed):** -O0 frame bloat for the same function (48B + 5 callee-saves for 2 locals) halves the recursion depth vs gcc -O0 (ccc dies at 200k, gcc survives; both die at 400k) — already tracked in ideas/reduce_stack_frame_size*.txt.
