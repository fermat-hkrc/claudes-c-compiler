static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 650; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    cells[3] = cells[2] + 406;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    arr[0 % 6] = 669; cells[6] += arr[0 % 6];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 344u; }
    cells[2] = cells[6] + 553;
    arr[0 % 6] = 817; cells[2] += arr[0 % 6];
    cells[5] = cells[0] + 403;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 204u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    static int f2(int *p, int *q) { *p += 439; return *q * 2; }
    sum += f2(&cells[6], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }