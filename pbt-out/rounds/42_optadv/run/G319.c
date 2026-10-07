static int f1(int *p, int *q) { *p += 579; return *q * 2; }
static int f2(int *p, int *q) { *p += 376; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 964; cells[5] += arr[5 % 6];
    arr[4 % 6] = 624; cells[4] += arr[4 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 613u; }
    sum += f1(&cells[5], &cells[2]);
    arr[7 % 6] = 553; cells[6] += arr[7 % 6];
    sum += f2(&cells[7], &cells[3]);
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 936u; }
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 625u; }
    cells[2] = cells[1] + 788;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 562u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    cells[5] = cells[1] + 311;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 161u; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 637u; }
    cells[1] = cells[6] + 169;
    cells[5] = cells[7] + 932;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }