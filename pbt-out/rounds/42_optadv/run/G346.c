static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 754; cells[0] += arr[5 % 6];
    cells[0] = cells[4] + 302;
    cells[7] = cells[4] + 715;
    static int f1(int *p, int *q) { *p += 134; return *q * 2; }
    sum += f1(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 332u; }
    static int f2(int *p, int *q) { *p += 770; return *q * 2; }
    sum += f2(&cells[6], &cells[4]);
    static int f3(int *p, int *q) { *p += 292; return *q * 2; }
    sum += f3(&cells[2], &cells[6]);
    arr[7 % 6] = 579; cells[2] += arr[7 % 6];
    static int f4(int *p, int *q) { *p += 626; return *q * 2; }
    sum += f4(&cells[2], &cells[6]);
    cells[5] = cells[2] + 28;
    static int f5(int *p, int *q) { *p += 373; return *q * 2; }
    sum += f5(&cells[2], &cells[0]);
    static int f6(int *p, int *q) { *p += 375; return *q * 2; }
    sum += f6(&cells[6], &cells[6]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[7] = cells[0] + 18;
    static int f7(int *p, int *q) { *p += 192; return *q * 2; }
    sum += f7(&cells[6], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }