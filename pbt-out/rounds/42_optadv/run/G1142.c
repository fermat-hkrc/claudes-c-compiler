static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 456u; }
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 9u; }
    cells[3] = cells[7] + 327;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[4] = cells[2] + 878;
    static int f1(int *p, int *q) { *p += 310; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[0] = cells[0] + 330;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 968u; }
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[5] = cells[3] + 19;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[7] = cells[7] + 115;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 366u; }
    static int f2(int *p, int *q) { *p += 458; return *q * 2; }
    sum += f2(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }