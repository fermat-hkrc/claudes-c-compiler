static int f1(int *p, int *q) { *p += 911; return *q * 2; }
static int f2(int *p, int *q) { *p += 405; return *q * 2; }
static int f3(int *p, int *q) { *p += 368; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[4] = cells[2] + 877;
    arr[6 % 6] = 616; cells[4] += arr[6 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[6 % 6] = 87; cells[0] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 832u; }
    sum += f1(&cells[0], &cells[3]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 927u; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 353u; }
    cells[7] = cells[2] + 493;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[1 % 6] = 981; cells[5] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f2(&cells[3], &cells[1]);
    sum += f3(&cells[1], &cells[5]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 17u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[4] = cells[3] + 486;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }