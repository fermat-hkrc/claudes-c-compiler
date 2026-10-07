static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 984; cells[3] += arr[2 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 198u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 367u; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 833u; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 937u; }
    cells[2] = cells[5] + 946;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[3] + 502;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 558; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 21u; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 745; return *q * 2; }
    sum += f2(&cells[1], &cells[6]);
    static int f3(int *p, int *q) { *p += 672; return *q * 2; }
    sum += f3(&cells[1], &cells[6]);
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 592u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }