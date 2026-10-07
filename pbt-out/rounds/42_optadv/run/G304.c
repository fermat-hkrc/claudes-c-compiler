static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 470u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 711u; }
    cells[1] = cells[1] + 822;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[4] = cells[6] + 327;
    cells[5] = cells[1] + 713;
    arr[7 % 6] = 4; cells[3] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[3 % 6] = 820; cells[3] += arr[3 % 6];
    cells[6] = cells[1] + 937;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f1(int *p, int *q) { *p += 418; return *q * 2; }
    sum += f1(&cells[4], &cells[4]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 565u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f2(int *p, int *q) { *p += 322; return *q * 2; }
    sum += f2(&cells[0], &cells[7]);
    static int f3(int *p, int *q) { *p += 705; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }