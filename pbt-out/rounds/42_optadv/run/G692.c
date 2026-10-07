static int f1(int *p, int *q) { *p += 679; return *q * 2; }
static int f2(int *p, int *q) { *p += 17; return *q * 2; }
static int f3(int *p, int *q) { *p += 950; return *q * 2; }
static int f4(int *p, int *q) { *p += 725; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f1(&cells[3], &cells[1]);
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 791u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    cells[6] = cells[5] + 643;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[0], &cells[4]);
    arr[7 % 6] = 367; cells[1] += arr[7 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 294u; }
    sum += f3(&cells[5], &cells[6]);
    cells[3] = cells[0] + 68;
    sum += f4(&cells[1], &cells[3]);
    arr[4 % 6] = 307; cells[7] += arr[4 % 6];
    cells[7] = cells[2] + 376;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }