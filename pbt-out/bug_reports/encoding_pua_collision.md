# Bug: PUA encoding silently corrupts source files containing literal U+E080..U+E0FF characters

**Found by:** `pi-pbt build-run --provider xai --model grok-4.6 --lang en --build-cmd "cargo check --lib" --scope src/common/encoding.rs --func bytes_to_string/decode_pua_byte` (PBT campaign on 2026-09-15, property P6 `pua_roundtrip`, proptest 1000 cases; collision precondition filtered out of P6 and confirmed by direct code-path analysis)

**Law:** `bytes_to_string` then lexer-side `decode_pua_byte` must preserve the original bytes of any
accepted input (module doc, src/common/encoding.rs:8-10: "encode non-UTF-8 bytes using Unicode Private
Use Area (PUA) code points, then decode them back to raw bytes in the lexer").

**Impact:** A valid UTF-8 C source file that legitimately contains a Private Use Area character in the
range U+E080..U+E0FF (e.g. inside a string literal — these are exactly the code points CCC chose for its
encoding scheme) is silently corrupted during lexing: the 3-byte UTF-8 sequence for such a character is
decoded back to a single byte 0x80..0xFF. The string literal content changes without any diagnostic.
This is silent data loss on input the API accepts (`bytes_to_string` takes any `Vec<u8>`).

**Function:** `ccc::common::encoding::{bytes_to_string, decode_pua_byte}` (src/common/encoding.rs:22, :80)

**Detected by:** Algebraic — Round-trip (encode∘decode = identity); the property was scoped to exclude
the collision range, whose members then fail the unscoped law (3 input bytes → 1 output byte).

**Minimal input:** byte vector `[0xEE, 0x82, 0x80]` (UTF-8 encoding of U+E080):
1. `bytes_to_string([0xEE,0x82,0x80])` → the string `"\u{E080}"` (valid UTF-8 is passed through unchanged).
2. The lexer recovers raw bytes via `decode_pua_byte("\u{E080}".as_bytes(), 0)` → returns `(0x80, 3)`
   (src/common/encoding.rs:90-96 matches `EE 82 80` as a PUA-encoded byte).
3. Round-trip result: `[0x80]` — the original 3 bytes are gone.

**Expected:** Either the decode step must distinguish "bytes that were PUA-encoded by us" from "bytes
that were valid UTF-8 all along" (impossible with the current scheme without extra state), or the
limitation must be documented in the module contract and — ideally — such inputs rejected/warned.

**Actual:** Silent 3-bytes→1-byte corruption for any source containing U+E080..U+E0FF (also affects
inputs whose bytes merely happen to form those UTF-8 sequences, e.g. comments in EUC-KR-adjacent files).

**Severity:** low (PUA characters in real-world C sources are rare; but the failure mode is silent data
loss, and the scheme's own reserved range guarantees the collision class exists)

**Regression test:** (none yet — P6 in `src/common/pbt_tests/encoding_pbt.rs` currently filters the
collision range; an explicit deterministic witness can be added on decision)

**Note:** This is an inherent limitation of PUA-transcoding schemes (GCC/Clang keep raw bytes and never
transcode). Filing as low-severity so the team can decide between documenting the limitation or
adding an escape mechanism (e.g. escaping literal PUA chars on encode).
