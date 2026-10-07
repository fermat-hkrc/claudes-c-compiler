static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 832u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f1(int *p, int *q) { *p += 797; return *q * 2; }
    sum += f1(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[3] + 604;
    static int f2(int *p, int *q) { *p += 375; return *q * 2; }
    sum += f2(&cells[0], &cells[3]);
    static int f3(int *p, int *q) { *p += 278; return *q * 2; }
    sum += f3(&cells[2], &cells[0]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    static int f4(int *p, int *q) { *p += 367; return *q * 2; }
    sum += f4(&cells[7], &cells[5]);
    cells[2] = cells[7] + 89;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }