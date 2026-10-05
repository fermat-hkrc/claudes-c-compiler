# Bug: build_cfg records duplicate predecessor entries for multi-target terminators with equal labels
**Law:** build_cfg returns ONE control flow graph as a (preds, succs) CSR pair; the two adjacency lists must be exact transposes of each other with no duplicate entries: ∀ i,b: i ∈ succs[b-row] ⇔ b ∈ preds[i-row], with equal multiplicity.
**Impact:** CondBranch with true_label == false_label and Switch with duplicate case targets (both representable IR; `case 1: case 2:` in C naturally produces duplicate switch labels) push the predecessor entry once per label while the successor entry is deduplicated. Every consumer that counts predecessors is corrupted: `if_convert.rs:305/386` (`preds.len(...) != 2` diamond detection), GVN's join detection (`gvn.rs:439 preds.len > 1`), mem2reg's dominance-frontier join gate and phi-copy cost estimate. Today the effects are masked (single-unique-pred blocks have idom == that pred, so the runner walk is a no-op; if_convert folds equal-target CondBranch first), but the data-structure invariant that six passes build on is violated — a latent correctness trap.
**Function:** build_cfg
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/ir/analysis.rs:110
**Detected by:** Algebraic invariant (transpose consistency), property P9
**Minimal input:** block0: CondBranch{cond: 1, true: L1, false: L1}; block1: Return
**Expected:** succs[0] = [1], preds[1] = [0]
**Actual:** succs[0] = [1], preds[1] = [0, 0] — the `preds[f].push(i32)` in the CondBranch arm runs unconditionally while the succs push is guarded by `contains`
**Severity:** medium (invariant violation feeding six passes; currently masked)
**Doc contract:** src/ir/analysis.rs:106 "Build predecessor and successor lists from the function's CFG. Returns (preds, succs) as flat adjacency lists (CSR format)." (fingerprint 767abe89)
**Fix:** Deduplicate the preds push symmetrically to the succs push (or dedupe both during the Vec<Vec>→CSR flatten); the same unconditional push exists in the Switch and IndirectBranch arms.
**Regression test:** src/ir/analysis.rs `pbt_regression::test_ir_analysis_regression_build_cfg_dup_preds`

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::analysis::pbt_regression::test_ir_analysis_regression_build_cfg_dup_preds
```
**Raw output:**
```
Test failed: duplicate predecessor 2 in row 2
minimal failing input: descs = [(0,0,0,0), (0,0,0,0), …]  (block 2 reached twice by one CondBranch with equal targets)
```
