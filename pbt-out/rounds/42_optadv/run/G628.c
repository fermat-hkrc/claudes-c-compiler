static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 46; cells[4] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 29; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    arr[1 % 6] = 971; cells[2] += arr[1 % 6];
    arr[2 % 6] = 283; cells[5] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f2(int *p, int *q) { *p += 298; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    static int f3(int *p, int *q) { *p += 625; return *q * 2; }
    sum += f3(&cells[7], &cells[5]);
    arr[3 % 6] = 175; cells[7] += arr[3 % 6];
    static int f4(int *p, int *q) { *p += 933; return *q * 2; }
    sum += f4(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f5(int *p, int *q) { *p += 581; return *q * 2; }
    sum += f5(&cells[5], &cells[4]);
    arr[6 % 6] = 144; cells[0] += arr[6 % 6];
    cells[6] = cells[5] + 150;
    cells[5] = cells[5] + 279;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    static int f6(int *p, int *q) { *p += 923; return *q * 2; }
    sum += f6(&cells[6], &cells[0]);
    static int f7(int *p, int *q) { *p += 217; return *q * 2; }
    sum += f7(&cells[5], &cells[3]);
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    arr[7 % 6] = 684; cells[7] += arr[7 % 6];
    cells[4] = cells[0] + 506;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 37u; }
    arr[5 % 6] = 325; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }