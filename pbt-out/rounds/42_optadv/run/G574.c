static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 355; cells[1] += arr[7 % 6];
    cells[6] = cells[2] + 933;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 896; cells[3] += arr[4 % 6];
    arr[5 % 6] = 511; cells[4] += arr[5 % 6];
    cells[2] = cells[4] + 276;
    cells[1] = cells[0] + 6;
    static int f1(int *p, int *q) { *p += 711; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    static int f2(int *p, int *q) { *p += 783; return *q * 2; }
    sum += f2(&cells[5], &cells[5]);
    static int f3(int *p, int *q) { *p += 308; return *q * 2; }
    sum += f3(&cells[0], &cells[7]);
    cells[4] = cells[0] + 379;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 948u; }
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    arr[1 % 6] = 653; cells[1] += arr[1 % 6];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 991u; }
    arr[7 % 6] = 432; cells[0] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }