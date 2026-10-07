static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    static int f1(int *p, int *q) { *p += 443; return *q * 2; }
    sum += f1(&cells[5], &cells[5]);
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 680u; }
    static int f2(int *p, int *q) { *p += 600; return *q * 2; }
    sum += f2(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    static int f3(int *p, int *q) { *p += 416; return *q * 2; }
    sum += f3(&cells[3], &cells[6]);
    static int f4(int *p, int *q) { *p += 422; return *q * 2; }
    sum += f4(&cells[7], &cells[7]);
    cells[4] = cells[6] + 760;
    static int f5(int *p, int *q) { *p += 703; return *q * 2; }
    sum += f5(&cells[0], &cells[2]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f6(int *p, int *q) { *p += 419; return *q * 2; }
    sum += f6(&cells[6], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 836u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 136; cells[4] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }