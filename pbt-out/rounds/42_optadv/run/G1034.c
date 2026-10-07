static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 937u; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 82u; }
    cells[6] = cells[5] + 681;
    static int f1(int *p, int *q) { *p += 361; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 899u; }
    cells[7] = cells[1] + 116;
    arr[0 % 6] = 62; cells[6] += arr[0 % 6];
    static int f2(int *p, int *q) { *p += 492; return *q * 2; }
    sum += f2(&cells[5], &cells[4]);
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 544u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }