static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[6 % 6] = 579; cells[4] += arr[6 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[7] = cells[5] + 751;
    arr[7 % 6] = 813; cells[6] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 101u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    static int f1(int *p, int *q) { *p += 826; return *q * 2; }
    sum += f1(&cells[4], &cells[2]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 317u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 926u; }
    static int f2(int *p, int *q) { *p += 888; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    cells[5] = cells[6] + 344;
    arr[6 % 6] = 960; cells[3] += arr[6 % 6];
    static int f3(int *p, int *q) { *p += 589; return *q * 2; }
    sum += f3(&cells[3], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }