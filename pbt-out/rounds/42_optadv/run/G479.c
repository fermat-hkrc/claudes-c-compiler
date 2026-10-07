static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    cells[1] = cells[2] + 335;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[4 % 6] = 181; cells[1] += arr[4 % 6];
    cells[5] = cells[5] + 308;
    arr[2 % 6] = 242; cells[2] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 871u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 643u; }
    static int f1(int *p, int *q) { *p += 874; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    arr[2 % 6] = 684; cells[5] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 979u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }