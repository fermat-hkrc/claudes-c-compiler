static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 400; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    arr[3 % 6] = 458; cells[3] += arr[3 % 6];
    arr[0 % 6] = 595; cells[5] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 535u; }
    cells[0] = cells[7] + 626;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 145u; }
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 575u; }
    static int f2(int *p, int *q) { *p += 917; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 988u; }
    static int f3(int *p, int *q) { *p += 286; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    arr[6 % 6] = 528; cells[5] += arr[6 % 6];
    cells[0] = cells[4] + 826;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    static int f4(int *p, int *q) { *p += 721; return *q * 2; }
    sum += f4(&cells[3], &cells[5]);
    arr[4 % 6] = 589; cells[2] += arr[4 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    arr[6 % 6] = 667; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 120u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }