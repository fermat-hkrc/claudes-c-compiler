static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 145; return *q * 2; }
    sum += f1(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[3] + 987;
    cells[6] = cells[4] + 657;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 405u; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[3 % 6] = 263; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 366u; }
    cells[0] = cells[4] + 980;
    static int f2(int *p, int *q) { *p += 473; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    cells[0] = cells[7] + 482;
    static int f3(int *p, int *q) { *p += 167; return *q * 2; }
    sum += f3(&cells[2], &cells[7]);
    cells[6] = cells[6] + 461;
    arr[6 % 6] = 358; cells[1] += arr[6 % 6];
    arr[0 % 6] = 821; cells[2] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }