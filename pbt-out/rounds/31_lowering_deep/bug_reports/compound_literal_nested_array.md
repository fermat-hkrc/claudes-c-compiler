# compound literal with nested array initializer drops row elements - &(int[3][3]){...} emits only each row's first element
> **Filed as [issue #1087](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1087)**

# Bug: nested-array compound literal data corruption

**Formal:** For every compound literal `(T[n][m]){ {r0...}, {r1...}, ... }` at global scope: the emitted backing object must equal gcc's byte-for-byte (all n*m elements). FALSIFIED: only each row's FIRST element is emitted; remaining elements are zero/garbage.

**Impact:** Silent data corruption for any address-taken 2-D (or nested) array compound literal:
```c
static int (*ca)[3] = &(int[3][3]){ {1,2,3}, {4,5,6}, {7,8,9} };
static int (*cb)[2] = &(int[2][2]){ {10,20}, {30,40} };
printf("cb=%d %d %d %d\n", cb[0][0], cb[0][1], cb[1][0], cb[1][1]);
  gcc: cb=10 20 30 40
  ccc: cb=10 0 30 0        <- 20 and 40 silently dropped
printf("ca1=%d %d %d\n", ca[1][0], ca[1][1], ca[1][2]);
  gcc: ca1=4 5 6
  ccc: ca1=0 7 0           <- backing data = [1,0,4,0,7,0] then uninitialized
                              (ca[2][2] reads into the ADJACENT global cp -> 10)
```
Backing-data reconstruction from reads: emitted words are `[1,0,4,0,7,0]` = each row contributes exactly ONE element + one zero, then the allocation trails uninitialized memory (reads leak the next global).

**Root cause:** `create_compound_literal_global` (src/ir/lowering/global_init.rs:1142) - for `TypeSpecifier::Array(elem_ts=int[3], _)` it takes `num_elems = items.len()` (=3 rows, alloc 36 bytes correct) but the nested per-row `Initializer::List` flattening in the subsequent `lower_global_init` path emits only each row's first scalar. Regular (non-compound) 2-D global initializers `int g[4][3] = {{...}}` are byte-exact vs gcc (verified in the same round, 300-case PBT), so the defect is specific to the compound-literal path.

**Separate from:** no existing issue mentions compound literals (dedup: "compound.literal" 0 hits across all ~1080 issue titles+bodies).

**Detected by:** round-31 ir/lowering deep PBT - property `p_compound_designated_statics_vs_gcc` (global compound literals + designated init + static locals), shrunk first case.

**Minimal input:**
```c
static int (*cb)[2] = &(int[2][2]){ {10,20}, {30,40} };
// cb[0][1] reads 0 instead of 20
```

**Fix sketch:** when `is_array` and the element type is itself an array, recurse the flattening per row (each row-List must emit all m elements at elem offsets), matching the non-compound path's handling.

## Found by
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- "p_compound_designated_statics" --nocapture
```
