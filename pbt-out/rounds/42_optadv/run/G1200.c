static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 607u; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[3] = cells[2] + 506;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 560u; }
    cells[1] = cells[0] + 300;
    static int f1(int *p, int *q) { *p += 351; return *q * 2; }
    sum += f1(&cells[1], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 153u; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 281u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[0 % 6] = 256; cells[7] += arr[0 % 6];
    arr[3 % 6] = 227; cells[5] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }