static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[6] + 805;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f1(int *p, int *q) { *p += 280; return *q * 2; }
    sum += f1(&cells[4], &cells[2]);
    cells[3] = cells[6] + 872;
    cells[2] = cells[4] + 765;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 304u; }
    cells[6] = cells[5] + 598;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 230u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[6 % 6] = 52; cells[4] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 129; return *q * 2; }
    sum += f2(&cells[5], &cells[0]);
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 178u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    static int f3(int *p, int *q) { *p += 600; return *q * 2; }
    sum += f3(&cells[1], &cells[1]);
    cells[7] = cells[4] + 974;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 97u; }
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 293; cells[6] += arr[7 % 6];
    cells[3] = cells[0] + 927;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }