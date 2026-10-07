static int f1(int *p, int *q) { *p += 982; return *q * 2; }
static int f2(int *p, int *q) { *p += 454; return *q * 2; }
static int f3(int *p, int *q) { *p += 315; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[4]);
    cells[0] = cells[2] + 115;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 974u; }
    cells[0] = cells[6] + 32;
    cells[4] = cells[4] + 295;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 97u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[3] + 244;
    sum += f2(&cells[3], &cells[5]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += f3(&cells[6], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }