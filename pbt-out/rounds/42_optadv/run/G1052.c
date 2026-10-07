static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 152; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 915u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 652u; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 491u; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 288u; }
    static int f2(int *p, int *q) { *p += 16; return *q * 2; }
    sum += f2(&cells[6], &cells[2]);
    static int f3(int *p, int *q) { *p += 237; return *q * 2; }
    sum += f3(&cells[4], &cells[5]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    cells[2] = cells[5] + 360;
    static int f4(int *p, int *q) { *p += 796; return *q * 2; }
    sum += f4(&cells[5], &cells[0]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    static int f5(int *p, int *q) { *p += 252; return *q * 2; }
    sum += f5(&cells[5], &cells[7]);
    static int f6(int *p, int *q) { *p += 464; return *q * 2; }
    sum += f6(&cells[5], &cells[1]);
    static int f7(int *p, int *q) { *p += 403; return *q * 2; }
    sum += f7(&cells[7], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }