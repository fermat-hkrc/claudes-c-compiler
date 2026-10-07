static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 252; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    arr[2 % 6] = 768; cells[5] += arr[2 % 6];
    cells[0] = cells[2] + 599;
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 372u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 717u; }
    arr[6 % 6] = 767; cells[5] += arr[6 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 873u; }
    cells[5] = cells[4] + 94;
    static int f1(int *p, int *q) { *p += 316; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    static int f2(int *p, int *q) { *p += 576; return *q * 2; }
    sum += f2(&cells[5], &cells[5]);
    static int f3(int *p, int *q) { *p += 84; return *q * 2; }
    sum += f3(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }