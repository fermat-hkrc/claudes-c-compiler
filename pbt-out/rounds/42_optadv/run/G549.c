static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 552; return *q * 2; }
    sum += f1(&cells[1], &cells[4]);
    static int f2(int *p, int *q) { *p += 648; return *q * 2; }
    sum += f2(&cells[0], &cells[6]);
    static int f3(int *p, int *q) { *p += 787; return *q * 2; }
    sum += f3(&cells[7], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 574u; }
    static int f4(int *p, int *q) { *p += 877; return *q * 2; }
    sum += f4(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 157u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 987u; }
    cells[0] = cells[3] + 518;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    cells[7] = cells[0] + 659;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f5(int *p, int *q) { *p += 706; return *q * 2; }
    sum += f5(&cells[7], &cells[6]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 557u; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 369u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[7 % 6] = 689; cells[4] += arr[7 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 480u; }
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }