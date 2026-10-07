# Round 44 — pragma/builtin/layout battery

## Bugs Found
### #1155 raw newline in string literal accepted
- Formal: ∀ string literal: no raw LF (C11 6.4.5p1). Witness: gcc "missing terminating"; clang 4 errors; ccc exit 0. Found by generator accident (ccc compiled AND ran the invalid source).

## Verified-correct (green, byte-exact)
- #pragma pack push/pop STACK semantics (1→2→pop→pop restores: 24/13/14/24/24)
- pack(1)×bitfields (PB=6, bytes ffffffff0151), natural layout NB=8
- __builtin memcpy/memset (8B and 16B/SSE paths), ffs family (0/0x10/0xFF00), bswap16
- heterogeneous union views (short/char[2]/bitfield trio), byte-view writes
- __atomic_fetch_add/store_n/load_n/compare_exchange_n with SEQ_CST/RELEASE/ACQUIRE/RELAXED orders
- 512-case giant switch at -O2 (jump-table limits), _Static_assert side-effect suppression
