struct S { int a; double b; char c[4]; };
struct BF { unsigned p:5; unsigned q:11; unsigned r:16; };
union U { unsigned u; unsigned char b[4]; struct BF bf; };
static unsigned h = 2166136261u;
static void mix(unsigned x) { h = (h ^ x) * 16777619u; }
static struct S mk(int s) { struct S r = { s, s * 1.5, {1,2,3,4} }; return r; }
static double use(struct S s) { return s.a * 2.0 + s.b + s.c[3]; }
static int recfn(int n) { return n <= 0 ? 1 : recfn(n - 1) * 2 % 100003; }
static int itfn(int n) { int r = 1; for (int i = 0; i < n; i++) r = r * 2 % 100003; return r; }
int cmain(int rec) {
    unsigned B = 5463595728746955192u, C = 1272092510u;
    int cells[8]; for (int i = 0; i < 8; i++) cells[i] = (int)((B >> (i * 4)) & 0xF) - 8;
    for (int i = 0; i < 8; i++) {
        struct S s = mk(cells[i] + (int)(C % 11));
        mix((unsigned)(use(s) * 4.0));
        union U u; u.u = B * (i + 1) + C;
        mix(u.b[i % 4] + u.bf.p);
        struct BF bf2 = { u.bf.q, u.bf.r, u.bf.p };
        mix((bf2.p << 16) + (bf2.q << 5) + bf2.r);
        switch (i % 5) {
            case 0: mix(bf2.p); break;
            case 1: mix(u.b[0] + 1); break;
            case 2: mix(cells[i] & 0xFFu); break;
            case 3: mix(C >> 3); break;
            default: mix(B + i); break;
        }
        int *p = &cells[(i * 3) % 8]; *p += (int)(u.bf.q % 7);
    }
    long t = 0;
    for (int k = 0; k < 6; k++) t = t * 3 + (long)(B >> (k * 5)) % 97;
    mix((unsigned)t);
    double d = (B % 997) / 7.0; float f = (float)d;
    mix((unsigned)(d * 1000.0) + (unsigned)(f * 100.0f) + (d < f ? 1 : 0));
    // v2 深化: 递归 + 函数指针 + 伪字符串哈希
    static int (*fp)(int);
    fp = rec ? &recfn : &itfn;
    mix((unsigned)fp((int)(C % 17)));
    char buf[16];
    for (int i = 0; i < 15; i++) buf[i] = (char)(48 + (i + (int)(B % 10)) % 10);
    buf[15] = 0;
    unsigned sh = 0; for (int i = 0; buf[i]; i++) sh = sh * 31 + (unsigned)buf[i];
    mix(sh);
    return (int)(h % 251);
}
int main(void){ return cmain(1); }