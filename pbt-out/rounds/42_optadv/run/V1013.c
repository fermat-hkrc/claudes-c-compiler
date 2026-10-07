static unsigned rotr32(unsigned x, unsigned r) { r &= 31; return r ? (x << (32 - r)) | (x >> r) : x; }
static int abs_b(int v) { int m = v >> 31; return (v ^ m) - m; }
static int maxb(int a, int b) { return a ^ ((a ^ b) & -(a < b)); }
static int minb(int a, int b) { return b ^ ((a ^ b) & -(a < b)); }
static int ovf_check(int a, int b, int *ok) {
    if (b > 0 && a > 2147483647 - b) { *ok = 0; return -1; }
    if (b < 0 && a < (-2147483647 - 1) - b) { *ok = 0; return -2; }
    *ok = 1; return a + b;
}
static unsigned strength(unsigned n, unsigned k) {
    unsigned s = 0;
    for (unsigned i = 0; i < n; i++) s += i * 8 + k;
    return s;
}
static long guarded_div(long a, long b) { return b ? a / b : -77; }
int cmain(int rec) {
    unsigned R = 31u;
    int A = 1354232116, B = 1354232116 + 1;
    unsigned X = 4033294678u, K = 2448171181u, N = 422866770 % 200u;
    long D = 3419103362978005091;
    long h = rotr32(X, R) + rotr32(X, R + 1) + rotr32(X, R + 31) + rotr32(X, 32);
    h += (long)abs_b(A) + abs_b(B) + abs_b(-2147483647 - 1);
    h += maxb(A, B) + minb(A, B) + maxb(-2147483647 - 1, B) * 0;
    int ok1, ok2, ok3;
    h += ovf_check(A, B, &ok1) + ovf_check(A, -B, &ok2) + ovf_check(2147483647, 1, &ok3) * ok3;
    h += (long)strength(N, K) + strength(N + 1, K + 7);
    h += guarded_div(D, D % 97 - 50) + guarded_div(D, 0);
    h += (long)(X - K) + (long)(K - X);   // 无符号回绕对称
    return (int)((h * 2654435761L) % 251);
}
int main(void){ return cmain(); }