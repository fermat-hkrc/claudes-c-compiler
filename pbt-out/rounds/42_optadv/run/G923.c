static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[6] + 876;
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 690; return *q * 2; }
    sum += f1(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[6 % 6] = 913; cells[4] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 964; return *q * 2; }
    sum += f2(&cells[4], &cells[6]);
    static int f3(int *p, int *q) { *p += 119; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 97u; }
    arr[0 % 6] = 914; cells[4] += arr[0 % 6];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 391u; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 911u; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 675u; }
    static int f4(int *p, int *q) { *p += 752; return *q * 2; }
    sum += f4(&cells[4], &cells[6]);
    cells[0] = cells[7] + 716;
    static int f5(int *p, int *q) { *p += 966; return *q * 2; }
    sum += f5(&cells[6], &cells[7]);
    cells[2] = cells[3] + 237;
    cells[6] = cells[2] + 795;
    static int f6(int *p, int *q) { *p += 168; return *q * 2; }
    sum += f6(&cells[1], &cells[6]);
    static int f7(int *p, int *q) { *p += 571; return *q * 2; }
    sum += f7(&cells[2], &cells[7]);
    cells[5] = cells[5] + 563;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }