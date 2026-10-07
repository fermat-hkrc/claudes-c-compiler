static int f1(int *p, int *q) { *p += 414; return *q * 2; }
static int f2(int *p, int *q) { *p += 821; return *q * 2; }
static int f3(int *p, int *q) { *p += 707; return *q * 2; }
static int f4(int *p, int *q) { *p += 188; return *q * 2; }
static int f5(int *p, int *q) { *p += 10; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 746u; }
    sum += f1(&cells[6], &cells[5]);
    sum += f2(&cells[7], &cells[4]);
    arr[4 % 6] = 504; cells[1] += arr[4 % 6];
    cells[2] = cells[2] + 396;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 557u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 935u; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f3(&cells[0], &cells[7]);
    sum += f4(&cells[4], &cells[7]);
    sum += f5(&cells[3], &cells[4]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 43u; }
    arr[4 % 6] = 457; cells[4] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 452u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }