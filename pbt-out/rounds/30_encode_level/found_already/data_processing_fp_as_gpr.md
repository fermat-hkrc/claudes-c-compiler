# Found Already: encode_add_sub/logical/div accept FP registers as GPRs

Found during round-30 adversarial encode PBT.
Root cause: get_reg calls parse_reg_num which extracts numeric suffix regardless
of register class prefix — same mechanism as #825 (parser accepts FP names as GPRs).
Mixed W/X also same as #791 (W/X width mixing).
No new issue filed — deduped to #825 and #791.
madd FP as addend → #825 (get_reg FP-as-GPR)
