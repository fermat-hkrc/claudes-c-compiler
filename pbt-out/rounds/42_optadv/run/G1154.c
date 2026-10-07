static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    static int f1(int *p, int *q) { *p += 877; return *q * 2; }
    sum += f1(&cells[5], &cells[7]);
    static int f2(int *p, int *q) { *p += 152; return *q * 2; }
    sum += f2(&cells[2], &cells[3]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    static int f3(int *p, int *q) { *p += 818; return *q * 2; }
    sum += f3(&cells[2], &cells[2]);
    cells[7] = cells[4] + 21;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 332u; }
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[0] + 82;
    cells[2] = cells[5] + 516;
    static int f4(int *p, int *q) { *p += 196; return *q * 2; }
    sum += f4(&cells[1], &cells[7]);
    static int f5(int *p, int *q) { *p += 671; return *q * 2; }
    sum += f5(&cells[3], &cells[4]);
    arr[2 % 6] = 534; cells[1] += arr[2 % 6];
    static int f6(int *p, int *q) { *p += 773; return *q * 2; }
    sum += f6(&cells[5], &cells[6]);
    cells[2] = cells[6] + 291;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f7(int *p, int *q) { *p += 900; return *q * 2; }
    sum += f7(&cells[6], &cells[0]);
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 367u; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }