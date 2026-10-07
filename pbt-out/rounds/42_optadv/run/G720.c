static int f1(int *p, int *q) { *p += 187; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 426u; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 240u; }
    arr[5 % 6] = 897; cells[7] += arr[5 % 6];
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 785u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    arr[4 % 6] = 671; cells[1] += arr[4 % 6];
    arr[3 % 6] = 46; cells[3] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 737u; }
    cells[5] = cells[3] + 185;
    sum += f1(&cells[0], &cells[6]);
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 384u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 76u; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[3 % 6] = 80; cells[1] += arr[3 % 6];
    cells[0] = cells[4] + 372;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 956u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    cells[7] = cells[2] + 693;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }