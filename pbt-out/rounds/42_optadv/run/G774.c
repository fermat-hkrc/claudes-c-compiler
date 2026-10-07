static int f1(int *p, int *q) { *p += 34; return *q * 2; }
static int f2(int *p, int *q) { *p += 675; return *q * 2; }
static int f3(int *p, int *q) { *p += 593; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 353; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 541u; }
    sum += f1(&cells[6], &cells[2]);
    cells[3] = cells[4] + 966;
    sum += f2(&cells[5], &cells[7]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    cells[1] = cells[4] + 332;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += f3(&cells[6], &cells[7]);
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 585u; }
    cells[5] = cells[0] + 825;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }