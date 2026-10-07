static int f1(int *p, int *q) { *p += 442; return *q * 2; }
static int f2(int *p, int *q) { *p += 40; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 204u; }
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f1(&cells[2], &cells[0]);
    cells[5] = cells[1] + 863;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[4] + 192;
    cells[0] = cells[5] + 893;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[4 % 6] = 548; cells[0] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 610u; }
    arr[0 % 6] = 348; cells[5] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 655u; }
    cells[4] = cells[4] + 636;
    arr[7 % 6] = 743; cells[7] += arr[7 % 6];
    sum += f2(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }