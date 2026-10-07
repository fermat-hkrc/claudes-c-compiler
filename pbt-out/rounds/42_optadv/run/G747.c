static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 465u; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    static int f1(int *p, int *q) { *p += 709; return *q * 2; }
    sum += f1(&cells[0], &cells[6]);
    cells[2] = cells[0] + 896;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 92u; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    cells[6] = cells[5] + 777;
    static int f2(int *p, int *q) { *p += 432; return *q * 2; }
    sum += f2(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 848; cells[7] += arr[1 % 6];
    static int f3(int *p, int *q) { *p += 899; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    static int f4(int *p, int *q) { *p += 965; return *q * 2; }
    sum += f4(&cells[0], &cells[5]);
    arr[2 % 6] = 863; cells[2] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[3 % 6] = 197; cells[1] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 706u; }
    static int f5(int *p, int *q) { *p += 452; return *q * 2; }
    sum += f5(&cells[0], &cells[2]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    static int f6(int *p, int *q) { *p += 298; return *q * 2; }
    sum += f6(&cells[3], &cells[6]);
    static int f7(int *p, int *q) { *p += 416; return *q * 2; }
    sum += f7(&cells[7], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }