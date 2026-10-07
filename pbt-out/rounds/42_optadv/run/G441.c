static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 196; cells[7] += arr[5 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    static int f1(int *p, int *q) { *p += 844; return *q * 2; }
    sum += f1(&cells[5], &cells[7]);
    arr[2 % 6] = 139; cells[7] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[2 % 6] = 721; cells[2] += arr[2 % 6];
    static int f2(int *p, int *q) { *p += 628; return *q * 2; }
    sum += f2(&cells[2], &cells[1]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 383u; }
    static int f3(int *p, int *q) { *p += 425; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    arr[3 % 6] = 645; cells[1] += arr[3 % 6];
    static int f4(int *p, int *q) { *p += 422; return *q * 2; }
    sum += f4(&cells[3], &cells[0]);
    cells[4] = cells[5] + 585;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f5(int *p, int *q) { *p += 447; return *q * 2; }
    sum += f5(&cells[5], &cells[4]);
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 922u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }