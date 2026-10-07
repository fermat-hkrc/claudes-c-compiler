static int f1(int *p, int *q) { *p += 619; return *q * 2; }
static int f2(int *p, int *q) { *p += 215; return *q * 2; }
static int f3(int *p, int *q) { *p += 113; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[1] + 663;
    sum += f1(&cells[4], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    cells[5] = cells[5] + 72;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 679u; }
    arr[1 % 6] = 33; cells[2] += arr[1 % 6];
    arr[1 % 6] = 856; cells[0] += arr[1 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    cells[2] = cells[4] + 848;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 351u; }
    sum += f2(&cells[2], &cells[6]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 143u; }
    cells[6] = cells[7] + 253;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 805; cells[5] += arr[3 % 6];
    sum += f3(&cells[4], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }