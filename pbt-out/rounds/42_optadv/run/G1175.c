static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 841; cells[5] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 223; return *q * 2; }
    sum += f1(&cells[1], &cells[1]);
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 419u; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    static int f2(int *p, int *q) { *p += 852; return *q * 2; }
    sum += f2(&cells[5], &cells[0]);
    cells[2] = cells[4] + 777;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    cells[6] = cells[0] + 351;
    static int f3(int *p, int *q) { *p += 20; return *q * 2; }
    sum += f3(&cells[4], &cells[5]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    arr[4 % 6] = 822; cells[1] += arr[4 % 6];
    cells[5] = cells[6] + 542;
    cells[0] = cells[0] + 526;
    cells[3] = cells[1] + 807;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 55u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 756u; }
    static int f4(int *p, int *q) { *p += 215; return *q * 2; }
    sum += f4(&cells[0], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }