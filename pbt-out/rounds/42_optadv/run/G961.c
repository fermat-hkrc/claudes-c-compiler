static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[2] = cells[7] + 616;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 612u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 307u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 164u; }
    static int f1(int *p, int *q) { *p += 919; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    static int f2(int *p, int *q) { *p += 346; return *q * 2; }
    sum += f2(&cells[3], &cells[1]);
    cells[0] = cells[5] + 740;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 365u; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f3(int *p, int *q) { *p += 431; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    static int f4(int *p, int *q) { *p += 207; return *q * 2; }
    sum += f4(&cells[0], &cells[1]);
    cells[6] = cells[1] + 918;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }