static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 248; return *q * 2; }
    sum += f1(&cells[5], &cells[0]);
    arr[3 % 6] = 697; cells[5] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 648; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    arr[7 % 6] = 745; cells[5] += arr[7 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[6 % 6] = 591; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    arr[7 % 6] = 815; cells[1] += arr[7 % 6];
    static int f3(int *p, int *q) { *p += 782; return *q * 2; }
    sum += f3(&cells[2], &cells[7]);
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    cells[7] = cells[4] + 452;
    static int f4(int *p, int *q) { *p += 372; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    static int f5(int *p, int *q) { *p += 803; return *q * 2; }
    sum += f5(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }