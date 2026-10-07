static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[4 % 6] = 949; cells[5] += arr[4 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    static int f1(int *p, int *q) { *p += 679; return *q * 2; }
    sum += f1(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    static int f2(int *p, int *q) { *p += 783; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 839u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 564u; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 632; cells[0] += arr[3 % 6];
    arr[0 % 6] = 942; cells[1] += arr[0 % 6];
    arr[6 % 6] = 795; cells[6] += arr[6 % 6];
    arr[4 % 6] = 625; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    static int f3(int *p, int *q) { *p += 775; return *q * 2; }
    sum += f3(&cells[1], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }