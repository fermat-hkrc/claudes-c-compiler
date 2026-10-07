static int f1(int *p, int *q) { *p += 251; return *q * 2; }
static int f2(int *p, int *q) { *p += 384; return *q * 2; }
static int f3(int *p, int *q) { *p += 696; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[6], &cells[4]);
    sum += f2(&cells[4], &cells[2]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 533u; }
    cells[5] = cells[1] + 674;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[2] = cells[6] + 828;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 905u; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 49u; }
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 278u; }
    arr[4 % 6] = 568; cells[3] += arr[4 % 6];
    arr[3 % 6] = 327; cells[7] += arr[3 % 6];
    sum += f3(&cells[7], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }