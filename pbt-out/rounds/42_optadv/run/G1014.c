static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    arr[7 % 6] = 234; cells[2] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 212u; }
    arr[1 % 6] = 566; cells[7] += arr[1 % 6];
    cells[7] = cells[4] + 823;
    cells[2] = cells[7] + 951;
    static int f1(int *p, int *q) { *p += 205; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    static int f2(int *p, int *q) { *p += 160; return *q * 2; }
    sum += f2(&cells[5], &cells[2]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 144u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 723u; }
    static int f3(int *p, int *q) { *p += 339; return *q * 2; }
    sum += f3(&cells[1], &cells[2]);
    cells[6] = cells[3] + 597;
    arr[4 % 6] = 152; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f4(int *p, int *q) { *p += 493; return *q * 2; }
    sum += f4(&cells[6], &cells[1]);
    static int f5(int *p, int *q) { *p += 867; return *q * 2; }
    sum += f5(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }