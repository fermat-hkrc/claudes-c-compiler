static int f1(int *p, int *q) { *p += 127; return *q * 2; }
static int f2(int *p, int *q) { *p += 500; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    arr[5 % 6] = 753; cells[7] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 389u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[3 % 6] = 550; cells[4] += arr[3 % 6];
    sum += f1(&cells[6], &cells[3]);
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 121u; }
    cells[2] = cells[0] + 549;
    cells[1] = cells[6] + 45;
    sum += f2(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    cells[1] = cells[2] + 104;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }