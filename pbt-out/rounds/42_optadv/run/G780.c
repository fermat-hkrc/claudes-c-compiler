static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[3 % 6] = 549; cells[2] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 265; return *q * 2; }
    sum += f1(&cells[4], &cells[1]);
    cells[3] = cells[7] + 968;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    static int f2(int *p, int *q) { *p += 913; return *q * 2; }
    sum += f2(&cells[4], &cells[6]);
    cells[0] = cells[0] + 416;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[5 % 6] = 78; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[2] + 761;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }