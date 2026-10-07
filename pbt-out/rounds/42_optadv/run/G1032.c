static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[6] + 695;
    static int f1(int *p, int *q) { *p += 613; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[4] + 780;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    static int f2(int *p, int *q) { *p += 213; return *q * 2; }
    sum += f2(&cells[2], &cells[5]);
    arr[4 % 6] = 182; cells[1] += arr[4 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 895u; }
    arr[4 % 6] = 798; cells[6] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }