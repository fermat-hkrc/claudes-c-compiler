static int f1(int *p, int *q) { *p += 871; return *q * 2; }
static int f2(int *p, int *q) { *p += 521; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[5] = cells[7] + 880;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    cells[7] = cells[0] + 74;
    cells[7] = cells[0] + 382;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 695u; }
    sum += f1(&cells[3], &cells[3]);
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 141u; }
    sum += f2(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[0 % 6] = 678; cells[5] += arr[0 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[3] + 528;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[0] = cells[3] + 142;
    arr[3 % 6] = 889; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }