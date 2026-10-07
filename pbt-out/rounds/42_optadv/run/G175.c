static int f1(int *p, int *q) { *p += 228; return *q * 2; }
static int f2(int *p, int *q) { *p += 597; return *q * 2; }
static int f3(int *p, int *q) { *p += 451; return *q * 2; }
static int f4(int *p, int *q) { *p += 268; return *q * 2; }
static int f5(int *p, int *q) { *p += 723; return *q * 2; }
static int f6(int *p, int *q) { *p += 130; return *q * 2; }
static int f7(int *p, int *q) { *p += 166; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[1]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[3] = cells[4] + 203;
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[4], &cells[5]);
    sum += f3(&cells[3], &cells[7]);
    sum += f4(&cells[7], &cells[0]);
    sum += f5(&cells[5], &cells[6]);
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[4] + 55;
    sum += f6(&cells[0], &cells[6]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 396u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f7(&cells[2], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }