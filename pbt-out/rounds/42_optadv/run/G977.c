static int f1(int *p, int *q) { *p += 501; return *q * 2; }
static int f2(int *p, int *q) { *p += 546; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    arr[6 % 6] = 479; cells[6] += arr[6 % 6];
    sum += f1(&cells[5], &cells[5]);
    cells[0] = cells[0] + 966;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[2] + 921;
    arr[6 % 6] = 116; cells[3] += arr[6 % 6];
    arr[7 % 6] = 896; cells[3] += arr[7 % 6];
    arr[6 % 6] = 981; cells[5] += arr[6 % 6];
    cells[2] = cells[1] + 886;
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[2 % 6] = 546; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[4], &cells[3]);
    arr[1 % 6] = 274; cells[1] += arr[1 % 6];
    cells[0] = cells[1] + 274;
    arr[2 % 6] = 864; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }