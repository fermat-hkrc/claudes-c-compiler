static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    cells[4] = cells[2] + 286;
    cells[5] = cells[2] + 324;
    static int f1(int *p, int *q) { *p += 15; return *q * 2; }
    sum += f1(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    cells[0] = cells[1] + 727;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    static int f2(int *p, int *q) { *p += 351; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    arr[0 % 6] = 698; cells[6] += arr[0 % 6];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    cells[2] = cells[4] + 621;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 887; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    static int f4(int *p, int *q) { *p += 204; return *q * 2; }
    sum += f4(&cells[0], &cells[7]);
    static int f5(int *p, int *q) { *p += 577; return *q * 2; }
    sum += f5(&cells[1], &cells[7]);
    arr[4 % 6] = 385; cells[2] += arr[4 % 6];
    arr[4 % 6] = 977; cells[2] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }