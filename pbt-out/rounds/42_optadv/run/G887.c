static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[0] + 417;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 263u; }
    arr[4 % 6] = 948; cells[6] += arr[4 % 6];
    cells[4] = cells[3] + 428;
    static int f1(int *p, int *q) { *p += 265; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[2 % 6] = 374; cells[5] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 930u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    cells[6] = cells[2] + 317;
    static int f2(int *p, int *q) { *p += 788; return *q * 2; }
    sum += f2(&cells[7], &cells[2]);
    cells[2] = cells[3] + 533;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }