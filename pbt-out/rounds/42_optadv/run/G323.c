static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 842; return *q * 2; }
    sum += f1(&cells[2], &cells[3]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    static int f2(int *p, int *q) { *p += 240; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 413u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[1] = cells[7] + 586;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f3(int *p, int *q) { *p += 989; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    arr[6 % 6] = 590; cells[1] += arr[6 % 6];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f4(int *p, int *q) { *p += 551; return *q * 2; }
    sum += f4(&cells[6], &cells[7]);
    static int f5(int *p, int *q) { *p += 754; return *q * 2; }
    sum += f5(&cells[6], &cells[0]);
    static int f6(int *p, int *q) { *p += 729; return *q * 2; }
    sum += f6(&cells[6], &cells[6]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    arr[5 % 6] = 475; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 886u; }
    static int f7(int *p, int *q) { *p += 886; return *q * 2; }
    sum += f7(&cells[2], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }