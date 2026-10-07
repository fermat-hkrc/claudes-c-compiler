static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 14; cells[7] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 203; return *q * 2; }
    sum += f1(&cells[2], &cells[3]);
    cells[6] = cells[0] + 718;
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 832u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    cells[4] = cells[5] + 102;
    static int f2(int *p, int *q) { *p += 699; return *q * 2; }
    sum += f2(&cells[6], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f3(int *p, int *q) { *p += 722; return *q * 2; }
    sum += f3(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f4(int *p, int *q) { *p += 860; return *q * 2; }
    sum += f4(&cells[6], &cells[7]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    static int f5(int *p, int *q) { *p += 35; return *q * 2; }
    sum += f5(&cells[6], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }