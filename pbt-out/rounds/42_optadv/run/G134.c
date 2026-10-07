static int f1(int *p, int *q) { *p += 203; return *q * 2; }
static int f2(int *p, int *q) { *p += 617; return *q * 2; }
static int f3(int *p, int *q) { *p += 781; return *q * 2; }
static int f4(int *p, int *q) { *p += 24; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[3] + 424;
    arr[0 % 6] = 121; cells[1] += arr[0 % 6];
    arr[2 % 6] = 413; cells[7] += arr[2 % 6];
    sum += f1(&cells[7], &cells[7]);
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    cells[5] = cells[7] + 780;
    sum += f2(&cells[2], &cells[1]);
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 417u; }
    sum += f3(&cells[6], &cells[4]);
    arr[2 % 6] = 262; cells[2] += arr[2 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[3] = cells[4] + 389;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    sum += f4(&cells[4], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 973u; }
    arr[4 % 6] = 490; cells[4] += arr[4 % 6];
    cells[4] = cells[1] + 396;
    cells[2] = cells[2] + 23;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }