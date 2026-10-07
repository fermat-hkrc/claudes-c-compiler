static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 876; cells[0] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 991u; }
    static int f1(int *p, int *q) { *p += 976; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    cells[7] = cells[3] + 336;
    cells[6] = cells[6] + 140;
    arr[1 % 6] = 356; cells[6] += arr[1 % 6];
    static int f2(int *p, int *q) { *p += 120; return *q * 2; }
    sum += f2(&cells[2], &cells[7]);
    arr[4 % 6] = 705; cells[1] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 164u; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 232u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 684u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[2] = cells[5] + 799;
    arr[0 % 6] = 507; cells[7] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }