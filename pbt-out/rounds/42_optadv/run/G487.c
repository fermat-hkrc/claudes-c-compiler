static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 553u; }
    cells[6] = cells[6] + 181;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 222u; }
    static int f1(int *p, int *q) { *p += 232; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    static int f2(int *p, int *q) { *p += 331; return *q * 2; }
    sum += f2(&cells[4], &cells[5]);
    static int f3(int *p, int *q) { *p += 34; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    cells[3] = cells[1] + 225;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }