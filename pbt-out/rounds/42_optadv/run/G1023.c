static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    arr[5 % 6] = 402; cells[1] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[4] = cells[1] + 714;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    static int f1(int *p, int *q) { *p += 285; return *q * 2; }
    sum += f1(&cells[5], &cells[1]);
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 983u; }
    arr[6 % 6] = 828; cells[0] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 621; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 597u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f3(int *p, int *q) { *p += 460; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f4(int *p, int *q) { *p += 526; return *q * 2; }
    sum += f4(&cells[6], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }