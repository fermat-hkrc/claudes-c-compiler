static int f1(int *p, int *q) { *p += 151; return *q * 2; }
static int f2(int *p, int *q) { *p += 922; return *q * 2; }
static int f3(int *p, int *q) { *p += 859; return *q * 2; }
static int f4(int *p, int *q) { *p += 724; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[5] + 834;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    sum += f1(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    arr[1 % 6] = 107; cells[3] += arr[1 % 6];
    cells[1] = cells[2] + 153;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    sum += f2(&cells[7], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += f3(&cells[6], &cells[5]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[6] = cells[6] + 902;
    cells[1] = cells[6] + 325;
    arr[4 % 6] = 182; cells[2] += arr[4 % 6];
    arr[1 % 6] = 127; cells[2] += arr[1 % 6];
    arr[1 % 6] = 826; cells[1] += arr[1 % 6];
    sum += f4(&cells[7], &cells[1]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }