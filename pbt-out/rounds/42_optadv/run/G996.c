static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[2 % 6] = 5; cells[2] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 571u; }
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[0 % 6] = 263; cells[7] += arr[0 % 6];
    static int f1(int *p, int *q) { *p += 639; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 103u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 575u; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 168u; }
    static int f2(int *p, int *q) { *p += 682; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    static int f3(int *p, int *q) { *p += 2; return *q * 2; }
    sum += f3(&cells[0], &cells[1]);
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    arr[1 % 6] = 133; cells[5] += arr[1 % 6];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 220u; }
    static int f4(int *p, int *q) { *p += 769; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f5(int *p, int *q) { *p += 297; return *q * 2; }
    sum += f5(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    cells[7] = cells[6] + 496;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }