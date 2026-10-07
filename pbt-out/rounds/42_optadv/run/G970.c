static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[4] = cells[3] + 218;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 846u; }
    arr[3 % 6] = 644; cells[5] += arr[3 % 6];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 489u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 191; cells[6] += arr[1 % 6];
    arr[1 % 6] = 383; cells[4] += arr[1 % 6];
    cells[6] = cells[3] + 594;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[5] = cells[5] + 397;
    cells[5] = cells[3] + 472;
    arr[6 % 6] = 911; cells[3] += arr[6 % 6];
    arr[3 % 6] = 711; cells[7] += arr[3 % 6];
    arr[5 % 6] = 540; cells[0] += arr[5 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    static int f1(int *p, int *q) { *p += 493; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    static int f2(int *p, int *q) { *p += 868; return *q * 2; }
    sum += f2(&cells[0], &cells[7]);
    static int f3(int *p, int *q) { *p += 248; return *q * 2; }
    sum += f3(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }