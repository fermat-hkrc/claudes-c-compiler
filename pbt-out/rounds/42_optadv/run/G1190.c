static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 780; return *q * 2; }
    sum += f1(&cells[4], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 49; return *q * 2; }
    sum += f2(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    arr[4 % 6] = 296; cells[2] += arr[4 % 6];
    cells[1] = cells[1] + 465;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[1 % 6] = 455; cells[4] += arr[1 % 6];
    arr[4 % 6] = 809; cells[7] += arr[4 % 6];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[0] = cells[7] + 499;
    cells[1] = cells[1] + 499;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[2] = cells[6] + 935;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f3(int *p, int *q) { *p += 875; return *q * 2; }
    sum += f3(&cells[1], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }