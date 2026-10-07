static int f1(int *p, int *q) { *p += 613; return *q * 2; }
static int f2(int *p, int *q) { *p += 863; return *q * 2; }
static int f3(int *p, int *q) { *p += 0; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[2]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 360u; }
    arr[1 % 6] = 454; cells[7] += arr[1 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    cells[4] = cells[1] + 108;
    sum += f2(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    cells[7] = cells[0] + 142;
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += f3(&cells[3], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }