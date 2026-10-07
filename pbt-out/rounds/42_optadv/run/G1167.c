static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 262; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 823u; }
    arr[6 % 6] = 635; cells[7] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 315; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    cells[0] = cells[5] + 559;
    static int f3(int *p, int *q) { *p += 517; return *q * 2; }
    sum += f3(&cells[1], &cells[1]);
    static int f4(int *p, int *q) { *p += 554; return *q * 2; }
    sum += f4(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 252; cells[2] += arr[1 % 6];
    static int f5(int *p, int *q) { *p += 604; return *q * 2; }
    sum += f5(&cells[0], &cells[3]);
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 966u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 848u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }