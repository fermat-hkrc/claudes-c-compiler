static int f1(int *p, int *q) { *p += 486; return *q * 2; }
static int f2(int *p, int *q) { *p += 393; return *q * 2; }
static int f3(int *p, int *q) { *p += 679; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f1(&cells[0], &cells[4]);
    cells[2] = cells[4] + 736;
    arr[0 % 6] = 310; cells[0] += arr[0 % 6];
    cells[2] = cells[2] + 300;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 508; cells[4] += arr[2 % 6];
    sum += f2(&cells[4], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[5 % 6] = 841; cells[7] += arr[5 % 6];
    cells[1] = cells[5] + 774;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 79; cells[1] += arr[7 % 6];
    cells[7] = cells[2] + 641;
    cells[2] = cells[0] + 437;
    sum += f3(&cells[2], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }