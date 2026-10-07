static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    static int f1(int *p, int *q) { *p += 544; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    static int f2(int *p, int *q) { *p += 133; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    static int f3(int *p, int *q) { *p += 433; return *q * 2; }
    sum += f3(&cells[6], &cells[0]);
    static int f4(int *p, int *q) { *p += 650; return *q * 2; }
    sum += f4(&cells[6], &cells[6]);
    arr[2 % 6] = 594; cells[3] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 724u; }
    arr[2 % 6] = 276; cells[2] += arr[2 % 6];
    cells[1] = cells[0] + 861;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 368u; }
    cells[4] = cells[7] + 743;
    static int f5(int *p, int *q) { *p += 70; return *q * 2; }
    sum += f5(&cells[5], &cells[0]);
    static int f6(int *p, int *q) { *p += 115; return *q * 2; }
    sum += f6(&cells[5], &cells[4]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }