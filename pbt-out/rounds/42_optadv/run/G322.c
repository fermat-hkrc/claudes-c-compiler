static int f1(int *p, int *q) { *p += 463; return *q * 2; }
static int f2(int *p, int *q) { *p += 194; return *q * 2; }
static int f3(int *p, int *q) { *p += 497; return *q * 2; }
static int f4(int *p, int *q) { *p += 403; return *q * 2; }
static int f5(int *p, int *q) { *p += 568; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 608u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 585u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[4] = cells[3] + 674;
    sum += f1(&cells[7], &cells[6]);
    cells[0] = cells[0] + 307;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    cells[4] = cells[2] + 928;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += f3(&cells[1], &cells[6]);
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[1 % 6] = 741; cells[5] += arr[1 % 6];
    sum += f4(&cells[7], &cells[5]);
    sum += f5(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }