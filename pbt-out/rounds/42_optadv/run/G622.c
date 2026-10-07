static int f1(int *p, int *q) { *p += 22; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[6] + 327;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 419; cells[2] += arr[0 % 6];
    arr[1 % 6] = 245; cells[5] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 545; cells[1] += arr[5 % 6];
    arr[6 % 6] = 413; cells[7] += arr[6 % 6];
    sum += f1(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    cells[1] = cells[6] + 190;
    cells[6] = cells[5] + 420;
    arr[7 % 6] = 118; cells[1] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }