static int f1(int *p, int *q) { *p += 498; return *q * 2; }
static int f2(int *p, int *q) { *p += 507; return *q * 2; }
static int f3(int *p, int *q) { *p += 428; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 666u; }
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[7], &cells[7]);
    sum += f2(&cells[5], &cells[5]);
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[1 % 6] = 363; cells[5] += arr[1 % 6];
    arr[2 % 6] = 344; cells[2] += arr[2 % 6];
    sum += f3(&cells[5], &cells[5]);
    arr[7 % 6] = 331; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    cells[4] = cells[4] + 735;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 521u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 862u; }
    arr[2 % 6] = 950; cells[6] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }