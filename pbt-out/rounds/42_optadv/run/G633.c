static int f1(int *p, int *q) { *p += 362; return *q * 2; }
static int f2(int *p, int *q) { *p += 1; return *q * 2; }
static int f3(int *p, int *q) { *p += 107; return *q * 2; }
static int f4(int *p, int *q) { *p += 435; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += f1(&cells[6], &cells[1]);
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 929u; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 845u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 564u; }
    arr[0 % 6] = 926; cells[3] += arr[0 % 6];
    arr[3 % 6] = 232; cells[1] += arr[3 % 6];
    arr[1 % 6] = 429; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 846; cells[0] += arr[3 % 6];
    sum += f2(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    arr[6 % 6] = 721; cells[7] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[6 % 6] = 858; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 37; cells[0] += arr[1 % 6];
    cells[2] = cells[1] + 697;
    sum += f3(&cells[0], &cells[5]);
    arr[4 % 6] = 100; cells[5] += arr[4 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f4(&cells[1], &cells[7]);
    cells[0] = cells[1] + 369;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }