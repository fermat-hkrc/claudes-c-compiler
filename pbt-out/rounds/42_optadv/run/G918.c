static int f1(int *p, int *q) { *p += 916; return *q * 2; }
static int f2(int *p, int *q) { *p += 635; return *q * 2; }
static int f3(int *p, int *q) { *p += 827; return *q * 2; }
static int f4(int *p, int *q) { *p += 126; return *q * 2; }
static int f5(int *p, int *q) { *p += 525; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 270u; }
    sum += f1(&cells[5], &cells[0]);
    sum += f2(&cells[6], &cells[0]);
    arr[6 % 6] = 974; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f3(&cells[3], &cells[0]);
    sum += f4(&cells[4], &cells[7]);
    sum += f5(&cells[6], &cells[5]);
    arr[0 % 6] = 393; cells[7] += arr[0 % 6];
    arr[1 % 6] = 17; cells[3] += arr[1 % 6];
    cells[2] = cells[2] + 403;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 965u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }