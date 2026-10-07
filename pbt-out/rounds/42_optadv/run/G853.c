static int f1(int *p, int *q) { *p += 474; return *q * 2; }
static int f2(int *p, int *q) { *p += 533; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 376u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += f1(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[1] + 332;
    arr[0 % 6] = 597; cells[5] += arr[0 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 136u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 564u; }
    cells[6] = cells[6] + 282;
    cells[5] = cells[3] + 401;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[2] + 150;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 272u; }
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 8u; }
    cells[6] = cells[0] + 976;
    sum += f2(&cells[5], &cells[4]);
    arr[5 % 6] = 998; cells[4] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 227; cells[5] += arr[5 % 6];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 811u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 791u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }