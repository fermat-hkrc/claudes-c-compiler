static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[2] + 119;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 86u; }
    static int f1(int *p, int *q) { *p += 498; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[4 % 6] = 406; cells[3] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 345; return *q * 2; }
    sum += f2(&cells[1], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    static int f3(int *p, int *q) { *p += 994; return *q * 2; }
    sum += f3(&cells[3], &cells[4]);
    cells[0] = cells[5] + 42;
    static int f4(int *p, int *q) { *p += 791; return *q * 2; }
    sum += f4(&cells[4], &cells[0]);
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 233u; }
    arr[6 % 6] = 872; cells[5] += arr[6 % 6];
    static int f5(int *p, int *q) { *p += 397; return *q * 2; }
    sum += f5(&cells[1], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }