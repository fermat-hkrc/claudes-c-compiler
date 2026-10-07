static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    static int f1(int *p, int *q) { *p += 875; return *q * 2; }
    sum += f1(&cells[1], &cells[5]);
    static int f2(int *p, int *q) { *p += 879; return *q * 2; }
    sum += f2(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[7 % 6] = 536; cells[0] += arr[7 % 6];
    static int f3(int *p, int *q) { *p += 23; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    arr[4 % 6] = 102; cells[7] += arr[4 % 6];
    cells[4] = cells[7] + 780;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f4(int *p, int *q) { *p += 925; return *q * 2; }
    sum += f4(&cells[4], &cells[3]);
    static int f5(int *p, int *q) { *p += 485; return *q * 2; }
    sum += f5(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    static int f6(int *p, int *q) { *p += 845; return *q * 2; }
    sum += f6(&cells[7], &cells[0]);
    arr[2 % 6] = 141; cells[4] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }