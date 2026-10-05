# Bug: dominance frontiers can never contain the entry block for itself (entry cycles)
**Law:** Per Cytron et al. 1991, DF_local(n) = {y ∈ succ(n) : n does not strictly dominate y}; equivalently DF(b) ∋ d iff b dominates a pred of d and b does not strictly dominate d. When the entry block is in ANY cycle (direct self-edge 0→0, or longer 0→…→p→0 with 0 dominating p), entry ∈ DF(entry).
**Impact:** Phi placement (mem2reg step 5) inserts phis at the iterated dominance frontiers of def blocks. With DF(entry) never containing entry, a variable stored inside such an entry cycle gets no phi at entry: the load placed before the store reads the initial zero every iteration instead of the previous iteration's value — a silent miscompile of the loop-carried value. Reachability today: the C frontend never targets the entry block (`lower_label_stmt` always terminates and starts a fresh block; no pass creates edges into entry), so this requires direct IR construction or a future pass that redirects a back edge into the entry block (e.g. block merging in cfg_simplify). Severity capped at low for that reason; the algorithm's contract violation is unconditional.
**Function:** compute_dominance_frontiers
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/ir/analysis.rs:302
**Detected by:** Reference — set-definition checker (property P11 found it; deterministic witness p11b)
**Minimal input:** single block, succs = [[0]] (entry self-loop); or 0→1, 1→{0,1}
**Expected:** DF(0) ∋ 0
**Actual:** DF(0) = {} — two compounding causes: (1) the `preds.len(b) < 2 → continue` join gate skips single-pred blocks, and (2) the runner walk stops at idom[entry] == entry, so even without the gate nothing is added
**Severity:** low (documented-unreachable from today's C lowering; filed for the definitional violation and future-pass hazard)
**Doc contract:** src/ir/analysis.rs:302 "Compute dominance frontiers for each block. DF(b) = set of blocks where b's dominance ends (join points)." (fingerprint 4b0f1f4b); Cytron et al. TOPLAS 1991 §2 (DF_local).
**Fix:** Add the local-frontier pass: for every block b and successor y, if b does not strictly dominate y, insert y into DF(b). This subsumes the runner walk's entry case; alternatively special-case `runner == idom[runner]` self-loops.
**Regression test:** src/ir/analysis.rs `pbt_tests::p11b_entry_selfloop_df` (intentionally red witness, both cycle forms)

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::analysis::pbt_tests::p11b_entry_selfloop_df
```
**Raw output:**
```
Test failed: DF(0) mismatch (n=1, succs=[[0]])
  left: `{}`, right: `{0}`
```
