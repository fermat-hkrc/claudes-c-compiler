static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 995; return *q * 2; }
    sum += f1(&cells[4], &cells[1]);
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 371u; }
    static int f2(int *p, int *q) { *p += 649; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    arr[2 % 6] = 963; cells[7] += arr[2 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    static int f3(int *p, int *q) { *p += 417; return *q * 2; }
    sum += f3(&cells[2], &cells[4]);
    arr[0 % 6] = 745; cells[5] += arr[0 % 6];
    arr[6 % 6] = 152; cells[3] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    static int f4(int *p, int *q) { *p += 985; return *q * 2; }
    sum += f4(&cells[4], &cells[2]);
    cells[5] = cells[5] + 504;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }