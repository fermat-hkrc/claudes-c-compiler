static int f1(int *p, int *q) { *p += 291; return *q * 2; }
static int f2(int *p, int *q) { *p += 260; return *q * 2; }
static int f3(int *p, int *q) { *p += 557; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[7]);
    arr[5 % 6] = 430; cells[6] += arr[5 % 6];
    arr[6 % 6] = 973; cells[0] += arr[6 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 863u; }
    cells[3] = cells[7] + 702;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[5], &cells[7]);
    cells[4] = cells[4] + 498;
    cells[2] = cells[6] + 11;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 959u; }
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 316; cells[2] += arr[0 % 6];
    cells[4] = cells[5] + 461;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 980u; }
    cells[2] = cells[4] + 595;
    cells[2] = cells[6] + 902;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }