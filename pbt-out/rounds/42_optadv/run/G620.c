static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 359u; }
    static int f1(int *p, int *q) { *p += 224; return *q * 2; }
    sum += f1(&cells[3], &cells[1]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 840u; }
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    static int f2(int *p, int *q) { *p += 943; return *q * 2; }
    sum += f2(&cells[1], &cells[3]);
    cells[3] = cells[1] + 138;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 192u; }
    arr[3 % 6] = 495; cells[4] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[7] + 57;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    static int f3(int *p, int *q) { *p += 540; return *q * 2; }
    sum += f3(&cells[3], &cells[5]);
    static int f4(int *p, int *q) { *p += 350; return *q * 2; }
    sum += f4(&cells[6], &cells[4]);
    static int f5(int *p, int *q) { *p += 648; return *q * 2; }
    sum += f5(&cells[1], &cells[3]);
    arr[7 % 6] = 647; cells[7] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }