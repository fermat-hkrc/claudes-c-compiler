static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 636; cells[2] += arr[4 % 6];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 631u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    arr[6 % 6] = 263; cells[1] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 0; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    cells[0] = cells[0] + 901;
    static int f2(int *p, int *q) { *p += 668; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    static int f3(int *p, int *q) { *p += 162; return *q * 2; }
    sum += f3(&cells[7], &cells[5]);
    static int f4(int *p, int *q) { *p += 671; return *q * 2; }
    sum += f4(&cells[4], &cells[7]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 236u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f5(int *p, int *q) { *p += 689; return *q * 2; }
    sum += f5(&cells[2], &cells[7]);
    cells[2] = cells[5] + 293;
    static int f6(int *p, int *q) { *p += 269; return *q * 2; }
    sum += f6(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[4 % 6] = 690; cells[7] += arr[4 % 6];
    arr[0 % 6] = 690; cells[4] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }