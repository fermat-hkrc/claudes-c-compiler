struct II { long a; int b; };              // 16B: rax+rdx
struct II mk(long a, int b) { struct II r = { a, b }; return r; }
long use(struct II v) { return v.a * 1000 + v.b; }
int main(void) { struct II v = mk(12, 34); printf("%ld %ld\n", use(v), use(mk(56, 78))); return 0; }
