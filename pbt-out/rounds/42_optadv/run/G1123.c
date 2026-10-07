static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 94; return *q * 2; }
    sum += f1(&cells[7], &cells[7]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    static int f2(int *p, int *q) { *p += 804; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[5 % 6] = 83; cells[2] += arr[5 % 6];
    static int f3(int *p, int *q) { *p += 120; return *q * 2; }
    sum += f3(&cells[4], &cells[0]);
    arr[7 % 6] = 252; cells[5] += arr[7 % 6];
    static int f4(int *p, int *q) { *p += 620; return *q * 2; }
    sum += f4(&cells[4], &cells[5]);
    cells[2] = cells[2] + 877;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 176u; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    static int f5(int *p, int *q) { *p += 406; return *q * 2; }
    sum += f5(&cells[1], &cells[7]);
    arr[4 % 6] = 596; cells[5] += arr[4 % 6];
    arr[2 % 6] = 281; cells[2] += arr[2 % 6];
    cells[6] = cells[0] + 540;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 110u; }
    static int f6(int *p, int *q) { *p += 918; return *q * 2; }
    sum += f6(&cells[4], &cells[5]);
    arr[7 % 6] = 360; cells[5] += arr[7 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[4 % 6] = 801; cells[2] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }