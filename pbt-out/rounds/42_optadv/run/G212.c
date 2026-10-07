static int f1(int *p, int *q) { *p += 950; return *q * 2; }
static int f2(int *p, int *q) { *p += 320; return *q * 2; }
static int f3(int *p, int *q) { *p += 614; return *q * 2; }
static int f4(int *p, int *q) { *p += 29; return *q * 2; }
static int f5(int *p, int *q) { *p += 676; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[6]);
    sum += f2(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 75u; }
    arr[3 % 6] = 989; cells[7] += arr[3 % 6];
    sum += f3(&cells[2], &cells[6]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f4(&cells[5], &cells[6]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[6] = cells[2] + 600;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[2] = cells[1] + 412;
    arr[2 % 6] = 794; cells[5] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 160u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[6 % 6] = 462; cells[3] += arr[6 % 6];
    arr[3 % 6] = 783; cells[4] += arr[3 % 6];
    cells[7] = cells[5] + 52;
    sum += f5(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }