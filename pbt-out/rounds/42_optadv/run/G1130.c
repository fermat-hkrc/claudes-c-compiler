static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f1(int *p, int *q) { *p += 779; return *q * 2; }
    sum += f1(&cells[0], &cells[2]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[0] + 761;
    static int f2(int *p, int *q) { *p += 116; return *q * 2; }
    sum += f2(&cells[1], &cells[7]);
    arr[4 % 6] = 917; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[4] = cells[5] + 872;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[0 % 6] = 960; cells[4] += arr[0 % 6];
    cells[7] = cells[2] + 108;
    cells[3] = cells[3] + 21;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    static int f3(int *p, int *q) { *p += 91; return *q * 2; }
    sum += f3(&cells[7], &cells[4]);
    arr[5 % 6] = 590; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[7] = cells[0] + 736;
    static int f4(int *p, int *q) { *p += 604; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }