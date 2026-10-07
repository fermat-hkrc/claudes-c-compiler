static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 120; return *q * 2; }
    sum += f1(&cells[2], &cells[1]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 61u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    cells[1] = cells[4] + 98;
    cells[1] = cells[2] + 603;
    arr[5 % 6] = 60; cells[6] += arr[5 % 6];
    static int f2(int *p, int *q) { *p += 703; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    cells[3] = cells[6] + 511;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 539u; }
    static int f3(int *p, int *q) { *p += 829; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }