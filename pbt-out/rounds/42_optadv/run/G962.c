static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 871; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    cells[7] = cells[4] + 719;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    static int f2(int *p, int *q) { *p += 307; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    cells[6] = cells[0] + 868;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    cells[0] = cells[0] + 919;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 797; cells[7] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 870u; }
    static int f3(int *p, int *q) { *p += 289; return *q * 2; }
    sum += f3(&cells[2], &cells[6]);
    cells[2] = cells[7] + 36;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    arr[0 % 6] = 508; cells[2] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }