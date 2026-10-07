static int f1(int *p, int *q) { *p += 513; return *q * 2; }
static int f2(int *p, int *q) { *p += 798; return *q * 2; }
static int f3(int *p, int *q) { *p += 813; return *q * 2; }
static int f4(int *p, int *q) { *p += 662; return *q * 2; }
static int f5(int *p, int *q) { *p += 892; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[4 % 6] = 473; cells[0] += arr[4 % 6];
    sum += f1(&cells[0], &cells[7]);
    sum += f2(&cells[0], &cells[0]);
    sum += f3(&cells[4], &cells[0]);
    cells[1] = cells[7] + 342;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f4(&cells[6], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[6] = cells[4] + 415;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 636u; }
    sum += f5(&cells[1], &cells[2]);
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[4 % 6] = 625; cells[7] += arr[4 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 259u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }