static int f1(int *p, int *q) { *p += 953; return *q * 2; }
static int f2(int *p, int *q) { *p += 50; return *q * 2; }
static int f3(int *p, int *q) { *p += 122; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[0]);
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 598u; }
    arr[6 % 6] = 746; cells[1] += arr[6 % 6];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 887u; }
    sum += f2(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[1] = cells[7] + 69;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 119u; }
    arr[3 % 6] = 42; cells[5] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[4 % 6] = 549; cells[6] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 675u; }
    sum += f3(&cells[2], &cells[7]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 436u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    cells[2] = cells[0] + 668;
    arr[6 % 6] = 647; cells[0] += arr[6 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 342u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 272u; }
    cells[1] = cells[5] + 99;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }