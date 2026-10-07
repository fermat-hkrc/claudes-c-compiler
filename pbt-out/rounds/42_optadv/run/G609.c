static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 58; return *q * 2; }
    sum += f1(&cells[2], &cells[3]);
    static int f2(int *p, int *q) { *p += 175; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    cells[1] = cells[3] + 847;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 476u; }
    cells[6] = cells[6] + 258;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 536u; }
    static int f3(int *p, int *q) { *p += 860; return *q * 2; }
    sum += f3(&cells[2], &cells[4]);
    static int f4(int *p, int *q) { *p += 164; return *q * 2; }
    sum += f4(&cells[6], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 972; cells[6] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    static int f5(int *p, int *q) { *p += 869; return *q * 2; }
    sum += f5(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 784u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }