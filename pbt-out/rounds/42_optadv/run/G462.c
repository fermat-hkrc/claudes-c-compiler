static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 587u; }
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 541u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 144u; }
    cells[7] = cells[7] + 562;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[4 % 6] = 391; cells[6] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[0] = cells[0] + 134;
    cells[3] = cells[6] + 841;
    static int f1(int *p, int *q) { *p += 585; return *q * 2; }
    sum += f1(&cells[5], &cells[0]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 790u; }
    cells[0] = cells[3] + 701;
    static int f2(int *p, int *q) { *p += 731; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }