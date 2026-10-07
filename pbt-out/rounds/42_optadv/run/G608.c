static int f1(int *p, int *q) { *p += 693; return *q * 2; }
static int f2(int *p, int *q) { *p += 856; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 131u; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += f1(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[6] = cells[6] + 438;
    sum += f2(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 492u; }
    arr[1 % 6] = 979; cells[7] += arr[1 % 6];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 871u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }