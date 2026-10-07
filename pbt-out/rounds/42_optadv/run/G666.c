static int f1(int *p, int *q) { *p += 207; return *q * 2; }
static int f2(int *p, int *q) { *p += 151; return *q * 2; }
static int f3(int *p, int *q) { *p += 53; return *q * 2; }
static int f4(int *p, int *q) { *p += 953; return *q * 2; }
static int f5(int *p, int *q) { *p += 236; return *q * 2; }
static int f6(int *p, int *q) { *p += 453; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[7] + 977;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 832u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 401u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[2], &cells[3]);
    arr[5 % 6] = 831; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    cells[4] = cells[0] + 560;
    cells[7] = cells[4] + 368;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 34u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 150u; }
    sum += f2(&cells[3], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[3], &cells[6]);
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 786u; }
    cells[0] = cells[2] + 95;
    sum += f4(&cells[2], &cells[3]);
    sum += f5(&cells[1], &cells[3]);
    sum += f6(&cells[3], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }