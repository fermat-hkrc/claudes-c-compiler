static int f1(int *p, int *q) { *p += 547; return *q * 2; }
static int f2(int *p, int *q) { *p += 849; return *q * 2; }
static int f3(int *p, int *q) { *p += 349; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 283u; }
    cells[0] = cells[6] + 653;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[5 % 6] = 542; cells[5] += arr[5 % 6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 839u; }
    cells[4] = cells[7] + 369;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 363u; }
    sum += f1(&cells[3], &cells[4]);
    cells[6] = cells[4] + 282;
    sum += f2(&cells[7], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 16u; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 767u; }
    sum += f3(&cells[4], &cells[2]);
    arr[4 % 6] = 133; cells[0] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 429; cells[6] += arr[2 % 6];
    arr[1 % 6] = 10; cells[3] += arr[1 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 193u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }