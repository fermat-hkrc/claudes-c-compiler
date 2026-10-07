static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[2] + 155;
    static int f1(int *p, int *q) { *p += 142; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    arr[6 % 6] = 398; cells[2] += arr[6 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    cells[3] = cells[7] + 668;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    static int f2(int *p, int *q) { *p += 127; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    arr[0 % 6] = 900; cells[7] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 833u; }
    cells[0] = cells[7] + 802;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    cells[7] = cells[4] + 199;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 86; cells[4] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }