static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 491; return *q * 2; }
    sum += f1(&cells[5], &cells[0]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 472u; }
    arr[4 % 6] = 2; cells[2] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 102; return *q * 2; }
    sum += f2(&cells[5], &cells[2]);
    static int f3(int *p, int *q) { *p += 982; return *q * 2; }
    sum += f3(&cells[1], &cells[4]);
    arr[1 % 6] = 549; cells[6] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[1 % 6] = 283; cells[7] += arr[1 % 6];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 851u; }
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    static int f4(int *p, int *q) { *p += 849; return *q * 2; }
    sum += f4(&cells[3], &cells[3]);
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 329u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 894u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 961u; }
    cells[2] = cells[7] + 818;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }