static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    arr[3 % 6] = 209; cells[4] += arr[3 % 6];
    cells[0] = cells[1] + 751;
    static int f1(int *p, int *q) { *p += 106; return *q * 2; }
    sum += f1(&cells[6], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f2(int *p, int *q) { *p += 490; return *q * 2; }
    sum += f2(&cells[0], &cells[6]);
    static int f3(int *p, int *q) { *p += 985; return *q * 2; }
    sum += f3(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 627u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 36u; }
    static int f4(int *p, int *q) { *p += 362; return *q * 2; }
    sum += f4(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[0 % 6] = 733; cells[7] += arr[0 % 6];
    cells[0] = cells[5] + 919;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 397u; }
    arr[4 % 6] = 648; cells[6] += arr[4 % 6];
    static int f5(int *p, int *q) { *p += 340; return *q * 2; }
    sum += f5(&cells[2], &cells[2]);
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 5u; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    static int f6(int *p, int *q) { *p += 877; return *q * 2; }
    sum += f6(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }