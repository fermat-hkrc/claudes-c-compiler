# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_add_sub | data_processing.rs | data_processing.rs | cargo test --lib | 5 passing / 3 failing properties (plus KAT + 3 regression witnesses) |
| cast_float_to_target | constants.rs | constants.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 regression witnesses) |
| classify_cast_with_f128 | cast.rs | cast.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 regression witnesses) |
