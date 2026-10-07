struct S_478 { float m0; float m1; };
int verify_mk_ok = 0;
struct S_478 mk(void) {
    struct S_478 s = { 0.250f, 1.250f };
    return s;
}
unsigned long long take(struct S_478 s) {
    unsigned long long chk = 1;
    chk &= s.m0 == (0.25f); chk &= s.m1 == (1.25f);
    verify_mk_ok = (int)chk;
    return chk * 0xdead01deULL + sizeof(s);
}