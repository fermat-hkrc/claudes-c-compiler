static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[1] + 221;
    static int f1(int *p, int *q) { *p += 249; return *q * 2; }
    sum += f1(&cells[6], &cells[0]);
    cells[4] = cells[4] + 661;
    cells[7] = cells[3] + 667;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 12u; }
    arr[5 % 6] = 854; cells[5] += arr[5 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[4] + 506;
    cells[6] = cells[4] + 968;
    cells[1] = cells[1] + 22;
    cells[4] = cells[1] + 320;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }