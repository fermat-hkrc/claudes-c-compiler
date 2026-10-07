static int f1(int *p, int *q) { *p += 61; return *q * 2; }
static int f2(int *p, int *q) { *p += 922; return *q * 2; }
static int f3(int *p, int *q) { *p += 623; return *q * 2; }
static int f4(int *p, int *q) { *p += 118; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 438u; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[0 % 6] = 928; cells[1] += arr[0 % 6];
    arr[2 % 6] = 115; cells[1] += arr[2 % 6];
    sum += f1(&cells[1], &cells[1]);
    cells[2] = cells[2] + 598;
    arr[3 % 6] = 688; cells[0] += arr[3 % 6];
    sum += f2(&cells[7], &cells[0]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[3 % 6] = 914; cells[6] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 174u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f3(&cells[4], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 589u; }
    sum += f4(&cells[0], &cells[4]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[4] = cells[2] + 969;
    arr[3 % 6] = 827; cells[2] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }