static int f1(int *p, int *q) { *p += 435; return *q * 2; }
static int f2(int *p, int *q) { *p += 978; return *q * 2; }
static int f3(int *p, int *q) { *p += 636; return *q * 2; }
static int f4(int *p, int *q) { *p += 379; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[5] = cells[7] + 912;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 261u; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[5] + 734;
    sum += f2(&cells[6], &cells[2]);
    arr[4 % 6] = 767; cells[1] += arr[4 % 6];
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 591u; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[6 % 6] = 883; cells[5] += arr[6 % 6];
    arr[0 % 6] = 392; cells[6] += arr[0 % 6];
    arr[4 % 6] = 17; cells[2] += arr[4 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 651u; }
    sum += f3(&cells[6], &cells[2]);
    arr[0 % 6] = 592; cells[3] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 801u; }
    sum += f4(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }