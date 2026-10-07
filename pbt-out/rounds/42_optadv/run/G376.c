static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    cells[0] = cells[2] + 22;
    static int f1(int *p, int *q) { *p += 374; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    cells[4] = cells[1] + 402;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[5 % 6] = 171; cells[2] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[1 % 6] = 856; cells[1] += arr[1 % 6];
    static int f2(int *p, int *q) { *p += 74; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[2] = cells[0] + 139;
    static int f3(int *p, int *q) { *p += 301; return *q * 2; }
    sum += f3(&cells[4], &cells[3]);
    arr[5 % 6] = 874; cells[3] += arr[5 % 6];
    cells[0] = cells[6] + 465;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    static int f4(int *p, int *q) { *p += 753; return *q * 2; }
    sum += f4(&cells[7], &cells[1]);
    arr[6 % 6] = 3; cells[6] += arr[6 % 6];
    static int f5(int *p, int *q) { *p += 428; return *q * 2; }
    sum += f5(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 479; cells[2] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }