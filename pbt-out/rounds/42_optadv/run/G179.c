static int f1(int *p, int *q) { *p += 568; return *q * 2; }
static int f2(int *p, int *q) { *p += 182; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    cells[1] = cells[0] + 393;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[4] + 29;
    cells[2] = cells[6] + 322;
    cells[5] = cells[6] + 59;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[6] = cells[6] + 913;
    cells[0] = cells[0] + 929;
    sum += f1(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    arr[3 % 6] = 940; cells[4] += arr[3 % 6];
    sum += f2(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[7] + 722;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }