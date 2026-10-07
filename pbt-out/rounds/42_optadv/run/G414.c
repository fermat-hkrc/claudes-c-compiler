static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[7] + 416;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[4] = cells[5] + 308;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 296u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[0 % 6] = 805; cells[3] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 724u; }
    cells[7] = cells[7] + 543;
    static int f1(int *p, int *q) { *p += 57; return *q * 2; }
    sum += f1(&cells[1], &cells[4]);
    static int f2(int *p, int *q) { *p += 531; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[3] = cells[3] + 218;
    arr[5 % 6] = 623; cells[7] += arr[5 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f3(int *p, int *q) { *p += 61; return *q * 2; }
    sum += f3(&cells[2], &cells[5]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 89u; }
    arr[4 % 6] = 668; cells[2] += arr[4 % 6];
    static int f4(int *p, int *q) { *p += 933; return *q * 2; }
    sum += f4(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }