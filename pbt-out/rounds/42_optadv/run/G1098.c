static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 640u; }
    arr[6 % 6] = 397; cells[7] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 771; cells[6] += arr[6 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 562u; }
    static int f1(int *p, int *q) { *p += 24; return *q * 2; }
    sum += f1(&cells[3], &cells[2]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    static int f2(int *p, int *q) { *p += 136; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    arr[0 % 6] = 661; cells[1] += arr[0 % 6];
    arr[6 % 6] = 623; cells[5] += arr[6 % 6];
    cells[3] = cells[3] + 536;
    arr[5 % 6] = 160; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }