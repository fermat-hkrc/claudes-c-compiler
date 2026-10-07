static int f1(int *p, int *q) { *p += 541; return *q * 2; }
static int f2(int *p, int *q) { *p += 260; return *q * 2; }
static int f3(int *p, int *q) { *p += 636; return *q * 2; }
static int f4(int *p, int *q) { *p += 104; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[7] = cells[5] + 373;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[3] + 415;
    cells[2] = cells[4] + 324;
    arr[6 % 6] = 16; cells[4] += arr[6 % 6];
    cells[3] = cells[5] + 853;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    cells[6] = cells[0] + 315;
    sum += f1(&cells[5], &cells[4]);
    arr[6 % 6] = 729; cells[1] += arr[6 % 6];
    cells[3] = cells[5] + 452;
    cells[4] = cells[1] + 487;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 966u; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += f2(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[4], &cells[4]);
    cells[2] = cells[7] + 334;
    sum += f4(&cells[7], &cells[6]);
    cells[2] = cells[4] + 63;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }