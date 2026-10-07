static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 298; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    static int f2(int *p, int *q) { *p += 302; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[6 % 6] = 704; cells[7] += arr[6 % 6];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 774u; }
    static int f3(int *p, int *q) { *p += 1; return *q * 2; }
    sum += f3(&cells[3], &cells[1]);
    static int f4(int *p, int *q) { *p += 767; return *q * 2; }
    sum += f4(&cells[2], &cells[2]);
    static int f5(int *p, int *q) { *p += 479; return *q * 2; }
    sum += f5(&cells[0], &cells[3]);
    arr[2 % 6] = 395; cells[6] += arr[2 % 6];
    static int f6(int *p, int *q) { *p += 261; return *q * 2; }
    sum += f6(&cells[3], &cells[1]);
    static int f7(int *p, int *q) { *p += 338; return *q * 2; }
    sum += f7(&cells[0], &cells[4]);
    cells[1] = cells[2] + 807;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    cells[3] = cells[3] + 271;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }