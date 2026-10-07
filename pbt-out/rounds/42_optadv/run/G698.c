static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[4] + 501;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    cells[4] = cells[5] + 709;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 586; cells[7] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 189u; }
    arr[6 % 6] = 606; cells[2] += arr[6 % 6];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 769u; }
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 210u; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    static int f1(int *p, int *q) { *p += 647; return *q * 2; }
    sum += f1(&cells[2], &cells[1]);
    static int f2(int *p, int *q) { *p += 24; return *q * 2; }
    sum += f2(&cells[2], &cells[3]);
    cells[2] = cells[6] + 686;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 573u; }
    arr[3 % 6] = 238; cells[4] += arr[3 % 6];
    static int f3(int *p, int *q) { *p += 409; return *q * 2; }
    sum += f3(&cells[1], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }