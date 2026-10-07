static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 218; cells[0] += arr[3 % 6];
    cells[3] = cells[5] + 44;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[2 % 6] = 523; cells[5] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 609; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 846u; }
    arr[2 % 6] = 940; cells[5] += arr[2 % 6];
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 547u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    cells[5] = cells[6] + 924;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[5] + 593;
    cells[2] = cells[4] + 601;
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 951; cells[3] += arr[1 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    static int f2(int *p, int *q) { *p += 227; return *q * 2; }
    sum += f2(&cells[3], &cells[1]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 664u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }