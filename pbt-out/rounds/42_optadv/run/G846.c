static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    static int f1(int *p, int *q) { *p += 812; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 893; return *q * 2; }
    sum += f2(&cells[4], &cells[5]);
    cells[6] = cells[2] + 319;
    arr[6 % 6] = 19; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 354u; }
    static int f3(int *p, int *q) { *p += 770; return *q * 2; }
    sum += f3(&cells[0], &cells[2]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 865u; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 707u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f4(int *p, int *q) { *p += 977; return *q * 2; }
    sum += f4(&cells[2], &cells[0]);
    static int f5(int *p, int *q) { *p += 427; return *q * 2; }
    sum += f5(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    static int f6(int *p, int *q) { *p += 576; return *q * 2; }
    sum += f6(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }