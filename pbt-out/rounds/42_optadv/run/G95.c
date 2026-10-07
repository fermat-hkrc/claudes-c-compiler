static int f1(int *p, int *q) { *p += 549; return *q * 2; }
static int f2(int *p, int *q) { *p += 667; return *q * 2; }
static int f3(int *p, int *q) { *p += 604; return *q * 2; }
static int f4(int *p, int *q) { *p += 989; return *q * 2; }
static int f5(int *p, int *q) { *p += 989; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 106u; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[0 % 6] = 609; cells[4] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 247; cells[4] += arr[3 % 6];
    cells[5] = cells[0] + 619;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[7 % 6] = 836; cells[0] += arr[7 % 6];
    sum += f1(&cells[4], &cells[5]);
    sum += f2(&cells[4], &cells[5]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 828u; }
    sum += f3(&cells[4], &cells[7]);
    sum += f4(&cells[4], &cells[7]);
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += f5(&cells[0], &cells[7]);
    arr[5 % 6] = 158; cells[7] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }