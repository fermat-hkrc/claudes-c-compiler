static int f1(int *p, int *q) { *p += 291; return *q * 2; }
static int f2(int *p, int *q) { *p += 181; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 180; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    arr[0 % 6] = 530; cells[2] += arr[0 % 6];
    sum += f1(&cells[3], &cells[5]);
    arr[4 % 6] = 990; cells[1] += arr[4 % 6];
    arr[2 % 6] = 600; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[7] + 264;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 804u; }
    cells[5] = cells[1] + 306;
    arr[0 % 6] = 722; cells[6] += arr[0 % 6];
    sum += f2(&cells[2], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 461u; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 102u; }
    cells[4] = cells[1] + 313;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[5] + 591;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 408u; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }