static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 431; cells[7] += arr[0 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    static int f1(int *p, int *q) { *p += 664; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    cells[6] = cells[5] + 984;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f2(int *p, int *q) { *p += 572; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    arr[4 % 6] = 529; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[3] + 987;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[4 % 6] = 936; cells[2] += arr[4 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }