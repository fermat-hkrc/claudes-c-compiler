static int f1(int *p, int *q) { *p += 108; return *q * 2; }
static int f2(int *p, int *q) { *p += 666; return *q * 2; }
static int f3(int *p, int *q) { *p += 859; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[0] = cells[2] + 716;
    sum += f1(&cells[6], &cells[4]);
    arr[4 % 6] = 81; cells[2] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 861u; }
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 750u; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    arr[6 % 6] = 934; cells[6] += arr[6 % 6];
    cells[5] = cells[7] + 58;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += f2(&cells[2], &cells[2]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[1 % 6] = 608; cells[1] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    arr[6 % 6] = 728; cells[2] += arr[6 % 6];
    sum += f3(&cells[5], &cells[5]);
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 120u; }
    cells[4] = cells[0] + 787;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }