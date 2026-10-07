static int f1(int *p, int *q) { *p += 575; return *q * 2; }
static int f2(int *p, int *q) { *p += 99; return *q * 2; }
static int f3(int *p, int *q) { *p += 528; return *q * 2; }
static int f4(int *p, int *q) { *p += 732; return *q * 2; }
static int f5(int *p, int *q) { *p += 39; return *q * 2; }
static int f6(int *p, int *q) { *p += 232; return *q * 2; }
static int f7(int *p, int *q) { *p += 897; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[1], &cells[5]);
    arr[0 % 6] = 798; cells[4] += arr[0 % 6];
    sum += f2(&cells[1], &cells[7]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 923u; }
    sum += f3(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 278u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 43u; }
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[3] = cells[1] + 856;
    sum += f4(&cells[2], &cells[4]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 940u; }
    arr[1 % 6] = 801; cells[5] += arr[1 % 6];
    sum += f5(&cells[6], &cells[2]);
    sum += f6(&cells[0], &cells[2]);
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 126u; }
    sum += f7(&cells[4], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }