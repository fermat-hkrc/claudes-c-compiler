static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 256; cells[2] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 779u; }
    static int f1(int *p, int *q) { *p += 31; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    cells[6] = cells[3] + 354;
    cells[2] = cells[2] + 421;
    static int f2(int *p, int *q) { *p += 169; return *q * 2; }
    sum += f2(&cells[0], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 146u; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    static int f3(int *p, int *q) { *p += 628; return *q * 2; }
    sum += f3(&cells[0], &cells[4]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[4] = cells[3] + 195;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }