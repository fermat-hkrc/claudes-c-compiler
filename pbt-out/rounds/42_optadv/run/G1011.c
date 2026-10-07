static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 564; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    static int f2(int *p, int *q) { *p += 602; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    static int f3(int *p, int *q) { *p += 739; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    static int f4(int *p, int *q) { *p += 659; return *q * 2; }
    sum += f4(&cells[1], &cells[0]);
    static int f5(int *p, int *q) { *p += 957; return *q * 2; }
    sum += f5(&cells[4], &cells[2]);
    static int f6(int *p, int *q) { *p += 736; return *q * 2; }
    sum += f6(&cells[0], &cells[6]);
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 29u; }
    static int f7(int *p, int *q) { *p += 455; return *q * 2; }
    sum += f7(&cells[6], &cells[2]);
    cells[7] = cells[5] + 981;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }