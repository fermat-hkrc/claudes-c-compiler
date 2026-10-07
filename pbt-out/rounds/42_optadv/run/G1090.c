static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[1] + 532;
    cells[6] = cells[7] + 873;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[5] = cells[3] + 293;
    arr[5 % 6] = 867; cells[0] += arr[5 % 6];
    arr[4 % 6] = 360; cells[5] += arr[4 % 6];
    static int f1(int *p, int *q) { *p += 462; return *q * 2; }
    sum += f1(&cells[6], &cells[3]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 757u; }
    arr[0 % 6] = 483; cells[0] += arr[0 % 6];
    cells[1] = cells[7] + 944;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 136u; }
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 368u; }
    cells[3] = cells[1] + 549;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 575u; }
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 719u; }
    static int f2(int *p, int *q) { *p += 476; return *q * 2; }
    sum += f2(&cells[7], &cells[5]);
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 807u; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 986u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }