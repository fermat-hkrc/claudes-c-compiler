static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[3] + 129;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 939u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    static int f1(int *p, int *q) { *p += 923; return *q * 2; }
    sum += f1(&cells[6], &cells[3]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 205u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[6] = cells[0] + 321;
    static int f2(int *p, int *q) { *p += 647; return *q * 2; }
    sum += f2(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 772u; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 938u; }
    arr[7 % 6] = 824; cells[7] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }