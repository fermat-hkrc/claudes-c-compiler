static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 184u; }
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[5 % 6] = 297; cells[5] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 921u; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[4] = cells[4] + 711;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    arr[2 % 6] = 575; cells[7] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 798u; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    static int f1(int *p, int *q) { *p += 138; return *q * 2; }
    sum += f1(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    static int f2(int *p, int *q) { *p += 774; return *q * 2; }
    sum += f2(&cells[0], &cells[4]);
    static int f3(int *p, int *q) { *p += 461; return *q * 2; }
    sum += f3(&cells[0], &cells[3]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 90u; }
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    cells[7] = cells[1] + 430;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }