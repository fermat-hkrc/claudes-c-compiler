static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    static int f1(int *p, int *q) { *p += 352; return *q * 2; }
    sum += f1(&cells[6], &cells[3]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[6] + 938;
    static int f2(int *p, int *q) { *p += 190; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    cells[5] = cells[4] + 100;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    arr[5 % 6] = 790; cells[7] += arr[5 % 6];
    cells[3] = cells[5] + 617;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 794u; }
    arr[2 % 6] = 267; cells[1] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 718; return *q * 2; }
    sum += f3(&cells[4], &cells[4]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 152u; }
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 420u; }
    static int f4(int *p, int *q) { *p += 165; return *q * 2; }
    sum += f4(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    static int f5(int *p, int *q) { *p += 25; return *q * 2; }
    sum += f5(&cells[1], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }