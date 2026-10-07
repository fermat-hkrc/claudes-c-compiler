static int f1(int *p, int *q) { *p += 316; return *q * 2; }
static int f2(int *p, int *q) { *p += 808; return *q * 2; }
static int f3(int *p, int *q) { *p += 134; return *q * 2; }
static int f4(int *p, int *q) { *p += 276; return *q * 2; }
static int f5(int *p, int *q) { *p += 816; return *q * 2; }
static int f6(int *p, int *q) { *p += 704; return *q * 2; }
static int f7(int *p, int *q) { *p += 300; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[5 % 6] = 542; cells[0] += arr[5 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += f2(&cells[0], &cells[5]);
    sum += f3(&cells[0], &cells[7]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 709u; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f4(&cells[0], &cells[4]);
    cells[4] = cells[5] + 456;
    sum += f5(&cells[1], &cells[6]);
    sum += f6(&cells[4], &cells[4]);
    sum += f7(&cells[0], &cells[0]);
    cells[5] = cells[6] + 139;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[4] = cells[3] + 495;
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 653u; }
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 855u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }