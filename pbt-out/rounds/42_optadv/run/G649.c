static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[4 % 6] = 617; cells[1] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f1(int *p, int *q) { *p += 84; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    cells[5] = cells[5] + 805;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 758; cells[4] += arr[3 % 6];
    cells[1] = cells[6] + 310;
    cells[6] = cells[3] + 712;
    cells[3] = cells[6] + 256;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 721u; }
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[5] = cells[1] + 806;
    static int f2(int *p, int *q) { *p += 933; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 847u; }
    arr[6 % 6] = 247; cells[7] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }