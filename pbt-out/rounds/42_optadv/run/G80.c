static int f1(int *p, int *q) { *p += 262; return *q * 2; }
static int f2(int *p, int *q) { *p += 275; return *q * 2; }
static int f3(int *p, int *q) { *p += 561; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[7] + 8;
    sum += f1(&cells[2], &cells[2]);
    arr[6 % 6] = 154; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 283u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    cells[4] = cells[1] + 923;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 889u; }
    sum += f2(&cells[4], &cells[6]);
    sum += f3(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }