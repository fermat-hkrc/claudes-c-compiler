static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[6] = cells[5] + 999;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    static int f1(int *p, int *q) { *p += 419; return *q * 2; }
    sum += f1(&cells[2], &cells[5]);
    arr[4 % 6] = 544; cells[1] += arr[4 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 599u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[4] + 438;
    cells[1] = cells[3] + 937;
    cells[7] = cells[1] + 145;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[4] = cells[1] + 309;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }