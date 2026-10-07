static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 712; cells[0] += arr[6 % 6];
    arr[5 % 6] = 156; cells[2] += arr[5 % 6];
    static int f1(int *p, int *q) { *p += 911; return *q * 2; }
    sum += f1(&cells[3], &cells[6]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    static int f2(int *p, int *q) { *p += 791; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[4] + 643;
    cells[4] = cells[4] + 465;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 107u; }
    arr[4 % 6] = 917; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }