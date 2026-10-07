static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 746; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 716; cells[3] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 267; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    static int f3(int *p, int *q) { *p += 532; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    arr[1 % 6] = 533; cells[1] += arr[1 % 6];
    arr[1 % 6] = 220; cells[5] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[5 % 6] = 588; cells[3] += arr[5 % 6];
    static int f4(int *p, int *q) { *p += 340; return *q * 2; }
    sum += f4(&cells[1], &cells[7]);
    arr[7 % 6] = 773; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    static int f5(int *p, int *q) { *p += 438; return *q * 2; }
    sum += f5(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[3] = cells[3] + 679;
    cells[4] = cells[2] + 977;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }