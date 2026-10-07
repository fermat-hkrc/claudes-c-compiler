static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    static int f1(int *p, int *q) { *p += 244; return *q * 2; }
    sum += f1(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[4 % 6] = 495; cells[0] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 832; return *q * 2; }
    sum += f2(&cells[6], &cells[4]);
    arr[2 % 6] = 411; cells[5] += arr[2 % 6];
    arr[3 % 6] = 164; cells[5] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f3(int *p, int *q) { *p += 788; return *q * 2; }
    sum += f3(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[1] = cells[7] + 632;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f4(int *p, int *q) { *p += 410; return *q * 2; }
    sum += f4(&cells[2], &cells[1]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[0 % 6] = 29; cells[2] += arr[0 % 6];
    cells[4] = cells[5] + 175;
    static int f5(int *p, int *q) { *p += 936; return *q * 2; }
    sum += f5(&cells[2], &cells[6]);
    arr[7 % 6] = 3; cells[2] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    static int f6(int *p, int *q) { *p += 930; return *q * 2; }
    sum += f6(&cells[4], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }