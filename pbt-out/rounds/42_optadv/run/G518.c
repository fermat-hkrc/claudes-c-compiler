static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 539u; }
    static int f1(int *p, int *q) { *p += 926; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f2(int *p, int *q) { *p += 75; return *q * 2; }
    sum += f2(&cells[2], &cells[5]);
    arr[1 % 6] = 345; cells[2] += arr[1 % 6];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    static int f3(int *p, int *q) { *p += 956; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 108u; }
    arr[4 % 6] = 856; cells[0] += arr[4 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 984u; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 652u; }
    arr[7 % 6] = 892; cells[2] += arr[7 % 6];
    cells[3] = cells[6] + 92;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 218u; }
    arr[1 % 6] = 861; cells[1] += arr[1 % 6];
    cells[6] = cells[7] + 669;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    static int f4(int *p, int *q) { *p += 153; return *q * 2; }
    sum += f4(&cells[5], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    static int f5(int *p, int *q) { *p += 644; return *q * 2; }
    sum += f5(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }