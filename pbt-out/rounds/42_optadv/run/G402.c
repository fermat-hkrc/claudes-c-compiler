static int f1(int *p, int *q) { *p += 775; return *q * 2; }
static int f2(int *p, int *q) { *p += 899; return *q * 2; }
static int f3(int *p, int *q) { *p += 947; return *q * 2; }
static int f4(int *p, int *q) { *p += 410; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[3] + 745;
    arr[4 % 6] = 193; cells[2] += arr[4 % 6];
    cells[7] = cells[0] + 398;
    sum += f1(&cells[6], &cells[5]);
    sum += f2(&cells[1], &cells[6]);
    arr[3 % 6] = 698; cells[4] += arr[3 % 6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[1] = cells[4] + 347;
    cells[4] = cells[6] + 421;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += f3(&cells[6], &cells[2]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 14u; }
    arr[1 % 6] = 908; cells[0] += arr[1 % 6];
    sum += f4(&cells[7], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }