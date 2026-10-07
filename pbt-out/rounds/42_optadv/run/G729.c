static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    static int f1(int *p, int *q) { *p += 36; return *q * 2; }
    sum += f1(&cells[5], &cells[1]);
    static int f2(int *p, int *q) { *p += 916; return *q * 2; }
    sum += f2(&cells[5], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[0] = cells[5] + 170;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 15u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 790u; }
    arr[2 % 6] = 99; cells[4] += arr[2 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    cells[7] = cells[0] + 932;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 781u; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 194u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 174u; }
    cells[5] = cells[0] + 153;
    cells[2] = cells[3] + 284;
    arr[2 % 6] = 513; cells[1] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 798; return *q * 2; }
    sum += f3(&cells[3], &cells[4]);
    arr[2 % 6] = 959; cells[2] += arr[2 % 6];
    static int f4(int *p, int *q) { *p += 439; return *q * 2; }
    sum += f4(&cells[0], &cells[3]);
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }