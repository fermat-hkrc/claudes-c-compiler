static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 176u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 208; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    static int f2(int *p, int *q) { *p += 282; return *q * 2; }
    sum += f2(&cells[6], &cells[3]);
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    arr[3 % 6] = 176; cells[5] += arr[3 % 6];
    static int f3(int *p, int *q) { *p += 427; return *q * 2; }
    sum += f3(&cells[5], &cells[0]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 344u; }
    static int f4(int *p, int *q) { *p += 484; return *q * 2; }
    sum += f4(&cells[7], &cells[4]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    static int f5(int *p, int *q) { *p += 503; return *q * 2; }
    sum += f5(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }