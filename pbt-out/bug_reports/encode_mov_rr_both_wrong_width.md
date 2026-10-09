# Bug: encode_mov_rr accepts both-operands wrong width (e.g. movl %al, %al)

**Law:** When both register operands disagree with the mnemonic size, MOV RR must return Err; it must not emit 88/89 via `reg_num` width aliasing.
**Impact:** Same class as mismatched-width: `movl %al, %al` assembles as `movl %eax, %eax` (`[0x89, 0xc0]`) with no diagnostic.
**Function:** encode_mov_rr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:162
**Detected by:** Negative/Error Contract (strengthen-round both-wrong-width generator)
**Minimal input:** `movl %al, %al`
**Expected:** `Err(...)`
**Actual:** `Ok([0x89, 0xc0])`
**Severity:** high
**Root cause:** `gp_integer.rs:179-191` — no `reg_size` check before `reg_num` + 88/89 (same defective path as mismatched-width).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;

        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0x88);
        } else {
            self.bytes.push(0x89);
        }
        self.bytes.push(self.modrm(3, src_num, dst_num));
        Ok(())
```
**Suggested fix:** Same gate as mismatched-width:
```rust
        if reg_size(&src.name) != size || reg_size(&dst.name) != size {
            return Err(format!(
                "mov register size mismatch: src={}, dst={}, expected size {}",
                src.name, dst.name, size
            ));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_rr_neg_both_wrong_width -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted both-wrong-width `movl %al, %al` → [89, c0].
minimal failing input: pair = 0, a_i = 0, b_i = 0
proptest seed: cc 7b19cc18f6043dfb5ca479fd1de321557b945c55e208a9c6c2812cccbff1fe37
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs (`encode_mov_rr_neg_both_wrong_width`)
