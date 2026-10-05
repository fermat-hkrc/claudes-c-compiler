int main(void){ _Static_assert(-((unsigned)(0 + -1)) == 1u, "neg wraps"); unsigned x = 4294967295u; _Static_assert((-x) == 1u, "neg2"); return 0; }
