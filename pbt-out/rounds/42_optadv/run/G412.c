static int f1(int *p, int *q) { *p += 940; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 24; cells[7] += arr[6 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 425u; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 780u; }
    cells[4] = cells[2] + 364;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 620u; }
    sum += f1(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 851u; }
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 970; cells[5] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 359u; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 578u; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 679u; }
    arr[3 % 6] = 866; cells[5] += arr[3 % 6];
    cells[6] = cells[5] + 238;
    arr[2 % 6] = 428; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }