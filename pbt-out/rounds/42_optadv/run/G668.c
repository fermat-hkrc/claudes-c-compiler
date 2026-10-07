static int f1(int *p, int *q) { *p += 821; return *q * 2; }
static int f2(int *p, int *q) { *p += 860; return *q * 2; }
static int f3(int *p, int *q) { *p += 216; return *q * 2; }
static int f4(int *p, int *q) { *p += 32; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 393u; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f1(&cells[5], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    arr[5 % 6] = 537; cells[1] += arr[5 % 6];
    sum += f2(&cells[1], &cells[3]);
    arr[0 % 6] = 973; cells[1] += arr[0 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[4] = cells[7] + 222;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += f3(&cells[0], &cells[6]);
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 124u; }
    sum += f4(&cells[0], &cells[2]);
    cells[6] = cells[5] + 476;
    cells[1] = cells[1] + 655;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }