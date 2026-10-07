static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[2] + 448;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 953; return *q * 2; }
    sum += f1(&cells[2], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 85u; }
    static int f2(int *p, int *q) { *p += 195; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 506u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    static int f3(int *p, int *q) { *p += 821; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    static int f4(int *p, int *q) { *p += 817; return *q * 2; }
    sum += f4(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    static int f5(int *p, int *q) { *p += 868; return *q * 2; }
    sum += f5(&cells[3], &cells[4]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 774u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }