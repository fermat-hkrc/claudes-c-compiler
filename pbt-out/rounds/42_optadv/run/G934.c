static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    static int f1(int *p, int *q) { *p += 438; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    static int f2(int *p, int *q) { *p += 247; return *q * 2; }
    sum += f2(&cells[2], &cells[6]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    static int f3(int *p, int *q) { *p += 849; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 786u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }