static int f1(int *p, int *q) { *p += 71; return *q * 2; }
static int f2(int *p, int *q) { *p += 985; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 940; cells[7] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 44u; }
    arr[2 % 6] = 540; cells[0] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[0] = cells[0] + 713;
    cells[1] = cells[1] + 997;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[6 % 6] = 236; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    arr[2 % 6] = 480; cells[3] += arr[2 % 6];
    arr[4 % 6] = 518; cells[5] += arr[4 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 298u; }
    arr[5 % 6] = 633; cells[2] += arr[5 % 6];
    arr[1 % 6] = 463; cells[7] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[2], &cells[3]);
    cells[6] = cells[4] + 736;
    arr[5 % 6] = 246; cells[2] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[4], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }