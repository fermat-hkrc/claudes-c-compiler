static int f1(int *p, int *q) { *p += 592; return *q * 2; }
static int f2(int *p, int *q) { *p += 133; return *q * 2; }
static int f3(int *p, int *q) { *p += 462; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    arr[5 % 6] = 8; cells[5] += arr[5 % 6];
    cells[3] = cells[7] + 782;
    cells[6] = cells[1] + 397;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 168; cells[3] += arr[1 % 6];
    sum += f1(&cells[7], &cells[3]);
    cells[4] = cells[6] + 566;
    sum += f2(&cells[7], &cells[7]);
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[4] = cells[4] + 126;
    sum += f3(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[0] + 86;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }