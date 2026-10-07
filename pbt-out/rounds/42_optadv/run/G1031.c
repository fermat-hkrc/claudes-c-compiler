static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 556u; }
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    arr[1 % 6] = 92; cells[2] += arr[1 % 6];
    static int f1(int *p, int *q) { *p += 812; return *q * 2; }
    sum += f1(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[4] = cells[5] + 421;
    static int f2(int *p, int *q) { *p += 78; return *q * 2; }
    sum += f2(&cells[7], &cells[7]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[6 % 6] = 145; cells[3] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    static int f3(int *p, int *q) { *p += 283; return *q * 2; }
    sum += f3(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }