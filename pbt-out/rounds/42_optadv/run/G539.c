static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 898; cells[1] += arr[6 % 6];
    cells[4] = cells[5] + 554;
    cells[2] = cells[3] + 522;
    cells[5] = cells[3] + 681;
    arr[4 % 6] = 453; cells[6] += arr[4 % 6];
    arr[2 % 6] = 990; cells[1] += arr[2 % 6];
    cells[2] = cells[6] + 863;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[6 % 6] = 757; cells[7] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 239; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    static int f2(int *p, int *q) { *p += 52; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 380u; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    arr[0 % 6] = 76; cells[7] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }