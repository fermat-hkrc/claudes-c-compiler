struct S { int a; double b; char c[4]; };
static unsigned h = 2166136261u;
static void mix(unsigned x) { h = (h ^ x) * 16777619u; }
static struct S mk(int s) { struct S r = { s, s * 1.5, {1,2,3,4} }; return r; }
static double use(struct S s) { return s.a * 2.0 + s.b + s.c[3]; }
static int recfn(int n) { return n <= 0 ? 1 : recfn(n - 1) * 2 % 100003; }
static int itfn(int n) { int r = 1; for (int i = 0; i < n; i++) r = r * 2 % 100003; return r; }
struct BF { unsigned p:5; unsigned q:11; };
int cmain(int rec) {
    unsigned B1 = 10957018010979369271u, B2 = 2551129556u;
    for (int i = 0; i < 8; i++) {
        struct S s = mk(i * 7 + (int)(B1 % 13));
        mix((unsigned)(use(s) * 4.0) ^ B1);
        struct BF bf = { i % 32, (B2 >> (i % 11)) % 2048 };
        mix((bf.p << 11) + bf.q + i);
        unsigned v[3] = { (unsigned)i, h, h >> 3 };
        mix(v[i % 3] + B2);
        switch (i) { case 2: mix(0xAA); break; case 5: mix(0xBB); break; default: mix(i + B1 % 7); }
    }
    union { unsigned u; unsigned char b[4]; } u = { B1 * 0x9E3779B9u };
    mix(u.b[2]);
    double d = (B1 % 1000) / 7.0; float f = (float)d;
    mix((unsigned)(d * 1000.0) + (unsigned)(f * 100.0f));
    long prod = 1;
    for (int k = 0; k < 6; k++) prod = prod * (k + (long)(B2 % 5) + 1) % 1000003L;
    mix((unsigned)prod);
    return (int)(h % 251);
}int main(void){ return cmain(0); }
