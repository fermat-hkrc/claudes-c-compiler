static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[3] = cells[4] + 455;
    arr[4 % 6] = 928; cells[5] += arr[4 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[5 % 6] = 239; cells[3] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 989u; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f1(int *p, int *q) { *p += 698; return *q * 2; }
    sum += f1(&cells[2], &cells[5]);
    arr[5 % 6] = 104; cells[1] += arr[5 % 6];
    static int f2(int *p, int *q) { *p += 731; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    cells[1] = cells[5] + 409;
    arr[5 % 6] = 903; cells[5] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f3(int *p, int *q) { *p += 542; return *q * 2; }
    sum += f3(&cells[0], &cells[7]);
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 314u; }
    arr[7 % 6] = 608; cells[1] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 733u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }