struct P { int a; long b; };             // 16B struct → 2 GP regs (SysV)
struct P mk(int a, long b) { struct P p = { a, b }; return p; }
long sum(struct P p) { return p.a + p.b; }
int main(void) { struct P p = mk(3, 4); printf("%ld\n", sum(p) + sum(mk(10, 20))); return 0; }
