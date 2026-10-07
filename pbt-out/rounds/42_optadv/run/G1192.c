static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 487; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    arr[1 % 6] = 551; cells[0] += arr[1 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[3 % 6] = 835; cells[5] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    arr[6 % 6] = 227; cells[7] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 144; return *q * 2; }
    sum += f2(&cells[6], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    static int f3(int *p, int *q) { *p += 55; return *q * 2; }
    sum += f3(&cells[0], &cells[2]);
    static int f4(int *p, int *q) { *p += 25; return *q * 2; }
    sum += f4(&cells[3], &cells[4]);
    arr[3 % 6] = 248; cells[0] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    cells[3] = cells[1] + 670;
    static int f5(int *p, int *q) { *p += 807; return *q * 2; }
    sum += f5(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    static int f6(int *p, int *q) { *p += 449; return *q * 2; }
    sum += f6(&cells[4], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }