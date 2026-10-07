static int f1(int *p, int *q) { *p += 389; return *q * 2; }
static int f2(int *p, int *q) { *p += 591; return *q * 2; }
static int f3(int *p, int *q) { *p += 20; return *q * 2; }
static int f4(int *p, int *q) { *p += 323; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[5] + 319;
    sum += f1(&cells[6], &cells[2]);
    arr[4 % 6] = 59; cells[5] += arr[4 % 6];
    cells[7] = cells[0] + 33;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += f2(&cells[1], &cells[0]);
    arr[7 % 6] = 482; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[0] = cells[0] + 388;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 789u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += f3(&cells[7], &cells[7]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 653u; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += f4(&cells[0], &cells[0]);
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }