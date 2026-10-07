static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 74; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    static int f2(int *p, int *q) { *p += 306; return *q * 2; }
    sum += f2(&cells[1], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[3 % 6] = 377; cells[0] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 405u; }
    cells[4] = cells[7] + 150;
    arr[5 % 6] = 293; cells[2] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 116u; }
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 610u; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 176u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 856u; }
    cells[7] = cells[5] + 522;
    arr[0 % 6] = 767; cells[4] += arr[0 % 6];
    static int f3(int *p, int *q) { *p += 326; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 885u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    arr[6 % 6] = 119; cells[0] += arr[6 % 6];
    cells[1] = cells[7] + 233;
    arr[1 % 6] = 940; cells[3] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }