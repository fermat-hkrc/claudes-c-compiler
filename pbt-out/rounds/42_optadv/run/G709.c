static int f1(int *p, int *q) { *p += 638; return *q * 2; }
static int f2(int *p, int *q) { *p += 556; return *q * 2; }
static int f3(int *p, int *q) { *p += 527; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[1 % 6] = 685; cells[7] += arr[1 % 6];
    sum += f1(&cells[5], &cells[0]);
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f2(&cells[3], &cells[6]);
    sum += f3(&cells[3], &cells[5]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    cells[0] = cells[2] + 482;
    cells[4] = cells[5] + 601;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    arr[0 % 6] = 126; cells[2] += arr[0 % 6];
    arr[4 % 6] = 336; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 672; cells[7] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }