static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 208; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[6] = cells[3] + 115;
    arr[6 % 6] = 704; cells[2] += arr[6 % 6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 318; cells[6] += arr[3 % 6];
    arr[5 % 6] = 977; cells[4] += arr[5 % 6];
    static int f1(int *p, int *q) { *p += 127; return *q * 2; }
    sum += f1(&cells[5], &cells[3]);
    static int f2(int *p, int *q) { *p += 661; return *q * 2; }
    sum += f2(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 41; cells[5] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    static int f3(int *p, int *q) { *p += 855; return *q * 2; }
    sum += f3(&cells[3], &cells[7]);
    cells[4] = cells[0] + 573;
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 410u; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 113u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }