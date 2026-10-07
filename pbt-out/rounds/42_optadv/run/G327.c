static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[0] + 904;
    arr[6 % 6] = 991; cells[1] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 796; return *q * 2; }
    sum += f1(&cells[6], &cells[4]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    static int f2(int *p, int *q) { *p += 124; return *q * 2; }
    sum += f2(&cells[2], &cells[3]);
    arr[7 % 6] = 273; cells[5] += arr[7 % 6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 478u; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    cells[6] = cells[3] + 657;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[2] = cells[1] + 814;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 247u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[4] + 384;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }