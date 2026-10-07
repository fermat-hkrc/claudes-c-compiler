static int f1(int *p, int *q) { *p += 846; return *q * 2; }
static int f2(int *p, int *q) { *p += 808; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[6] + 796;
    cells[6] = cells[1] + 489;
    cells[7] = cells[7] + 393;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[6], &cells[7]);
    cells[7] = cells[4] + 922;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[6 % 6] = 161; cells[5] += arr[6 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[1] + 454;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f2(&cells[7], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }