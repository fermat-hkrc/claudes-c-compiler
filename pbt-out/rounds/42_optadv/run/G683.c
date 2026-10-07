static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 911; return *q * 2; }
    sum += f1(&cells[0], &cells[1]);
    static int f2(int *p, int *q) { *p += 51; return *q * 2; }
    sum += f2(&cells[1], &cells[4]);
    static int f3(int *p, int *q) { *p += 874; return *q * 2; }
    sum += f3(&cells[4], &cells[3]);
    static int f4(int *p, int *q) { *p += 0; return *q * 2; }
    sum += f4(&cells[7], &cells[1]);
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 603u; }
    cells[5] = cells[4] + 16;
    cells[1] = cells[1] + 603;
    cells[5] = cells[4] + 261;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    arr[7 % 6] = 25; cells[2] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[7] = cells[0] + 343;
    static int f5(int *p, int *q) { *p += 368; return *q * 2; }
    sum += f5(&cells[2], &cells[5]);
    arr[0 % 6] = 318; cells[3] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 22u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }