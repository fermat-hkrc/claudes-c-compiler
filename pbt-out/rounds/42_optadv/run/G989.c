static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[7] + 86;
    arr[3 % 6] = 79; cells[3] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 989; return *q * 2; }
    sum += f1(&cells[7], &cells[0]);
    arr[4 % 6] = 929; cells[5] += arr[4 % 6];
    cells[1] = cells[5] + 689;
    cells[7] = cells[4] + 212;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 235; cells[3] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 892; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 191u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }