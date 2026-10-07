static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 497; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 310; cells[4] += arr[7 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    static int f1(int *p, int *q) { *p += 916; return *q * 2; }
    sum += f1(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[7 % 6] = 349; cells[4] += arr[7 % 6];
    cells[4] = cells[5] + 874;
    cells[5] = cells[6] + 565;
    cells[1] = cells[4] + 105;
    static int f2(int *p, int *q) { *p += 469; return *q * 2; }
    sum += f2(&cells[3], &cells[7]);
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 425u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[7 % 6] = 48; cells[2] += arr[7 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    static int f3(int *p, int *q) { *p += 845; return *q * 2; }
    sum += f3(&cells[3], &cells[2]);
    static int f4(int *p, int *q) { *p += 633; return *q * 2; }
    sum += f4(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 321u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }