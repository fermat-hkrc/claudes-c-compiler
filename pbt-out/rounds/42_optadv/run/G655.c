static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 518; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    static int f2(int *p, int *q) { *p += 485; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 439u; }
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 750u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 416u; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 263u; }
    static int f3(int *p, int *q) { *p += 10; return *q * 2; }
    sum += f3(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }