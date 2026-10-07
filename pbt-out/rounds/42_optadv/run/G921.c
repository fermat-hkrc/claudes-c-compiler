static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    arr[3 % 6] = 538; cells[2] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 625; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    static int f2(int *p, int *q) { *p += 563; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 160u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 405u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    cells[7] = cells[1] + 475;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }