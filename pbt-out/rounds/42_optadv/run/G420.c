static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    cells[4] = cells[3] + 883;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    static int f1(int *p, int *q) { *p += 457; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 478u; }
    static int f2(int *p, int *q) { *p += 517; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    cells[4] = cells[5] + 903;
    arr[0 % 6] = 588; cells[1] += arr[0 % 6];
    static int f3(int *p, int *q) { *p += 891; return *q * 2; }
    sum += f3(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[4] + 179;
    static int f4(int *p, int *q) { *p += 929; return *q * 2; }
    sum += f4(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }