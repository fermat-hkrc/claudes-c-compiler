static int f1(int *p, int *q) { *p += 240; return *q * 2; }
static int f2(int *p, int *q) { *p += 80; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[5]);
    cells[6] = cells[5] + 783;
    arr[6 % 6] = 522; cells[4] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[5 % 6] = 480; cells[2] += arr[5 % 6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    cells[7] = cells[6] + 858;
    cells[2] = cells[2] + 380;
    arr[0 % 6] = 685; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    arr[3 % 6] = 103; cells[7] += arr[3 % 6];
    sum += f2(&cells[4], &cells[4]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[4] = cells[6] + 477;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[7] + 646;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }