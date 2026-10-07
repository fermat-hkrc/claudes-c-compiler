static int f1(int *p, int *q) { *p += 274; return *q * 2; }
static int f2(int *p, int *q) { *p += 639; return *q * 2; }
static int f3(int *p, int *q) { *p += 722; return *q * 2; }
static int f4(int *p, int *q) { *p += 307; return *q * 2; }
static int f5(int *p, int *q) { *p += 577; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 717; cells[4] += arr[1 % 6];
    arr[6 % 6] = 261; cells[6] += arr[6 % 6];
    sum += f1(&cells[2], &cells[2]);
    arr[4 % 6] = 172; cells[4] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[2] = cells[6] + 821;
    sum += f2(&cells[7], &cells[1]);
    sum += f3(&cells[0], &cells[5]);
    arr[0 % 6] = 714; cells[7] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[3] = cells[3] + 802;
    arr[5 % 6] = 396; cells[3] += arr[5 % 6];
    sum += f4(&cells[2], &cells[3]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[1] = cells[4] + 869;
    sum += f5(&cells[6], &cells[7]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }