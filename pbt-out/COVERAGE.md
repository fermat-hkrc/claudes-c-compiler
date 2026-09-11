# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_add_sub | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 7 failing properties (plus KAT + 7 regression witnesses) |
| cast_float_to_target | constants.rs | constants.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 regression witnesses) |
| classify_cast_with_f128 | cast.rs | cast.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 regression witnesses) |
| encode_adc | data_processing.rs | data_processing.rs | cargo test --lib | 6 passing / 4 failing properties (plus KAT + 4 regression witnesses) |
| encode_adr | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 6 failing properties (plus 2 KAT + 6 regression witnesses) |
