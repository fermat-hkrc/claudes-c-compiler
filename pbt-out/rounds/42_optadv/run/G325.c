static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 492; cells[3] += arr[2 % 6];
    arr[4 % 6] = 138; cells[7] += arr[4 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 827u; }
    static int f1(int *p, int *q) { *p += 437; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 900u; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    arr[3 % 6] = 225; cells[2] += arr[3 % 6];
    cells[7] = cells[0] + 626;
    arr[4 % 6] = 792; cells[4] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }