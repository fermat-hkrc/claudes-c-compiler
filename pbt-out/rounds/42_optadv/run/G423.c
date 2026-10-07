static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[0] + 822;
    cells[0] = cells[2] + 679;
    static int f1(int *p, int *q) { *p += 811; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    cells[6] = cells[6] + 785;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f2(int *p, int *q) { *p += 421; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    cells[7] = cells[4] + 500;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[6] + 419;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    static int f3(int *p, int *q) { *p += 277; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    static int f4(int *p, int *q) { *p += 121; return *q * 2; }
    sum += f4(&cells[0], &cells[0]);
    static int f5(int *p, int *q) { *p += 283; return *q * 2; }
    sum += f5(&cells[4], &cells[7]);
    static int f6(int *p, int *q) { *p += 65; return *q * 2; }
    sum += f6(&cells[4], &cells[3]);
    arr[4 % 6] = 282; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    static int f7(int *p, int *q) { *p += 593; return *q * 2; }
    sum += f7(&cells[2], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }