# Bug: encode_orn ignores mismatched NEON arrangements and bare Reg sources
**Law:** Vector ORN requires Vd, Vn, Vm to share T ∈ {8B,16B}. llvm-mc rejects `orn v0.8b, v0.16b, v0.8b` and a bare GPR/V name in a vector operand.
**Impact:** `orn v0.8b, v0.16b, v0.8b` encodes as `orn v0.8b, v0.8b, v0.8b` (source arrangement discarded). `get_neon_reg` also accepts `Operand::Reg`, so a GPR can be encoded as Vm.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn v0.8b, v0.16b, v0.8b` (d=n=m=0, which=0)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — only dest arrangement is used for Q; Vn/Vm arrangements are discarded (`let (rn, _) = get_neon_reg`).
**Severity:** medium
**Regression test:** `test_encode_orn_regression_neon_mismatch` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** first parallel run failed with the same shrunk witness; encode_orn is a pure function (no shared state).
