static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[1] + 38;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 166u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[3] = cells[4] + 338;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 763; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f2(int *p, int *q) { *p += 488; return *q * 2; }
    sum += f2(&cells[1], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f3(int *p, int *q) { *p += 599; return *q * 2; }
    sum += f3(&cells[0], &cells[3]);
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[1 % 6] = 540; cells[2] += arr[1 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 516u; }
    arr[3 % 6] = 633; cells[4] += arr[3 % 6];
    cells[5] = cells[4] + 828;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[3] = cells[7] + 420;
    static int f4(int *p, int *q) { *p += 45; return *q * 2; }
    sum += f4(&cells[1], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }