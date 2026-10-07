static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 569; return *q * 2; }
    sum += f1(&cells[4], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f2(int *p, int *q) { *p += 580; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 492u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[6 % 6] = 59; cells[5] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 347u; }
    arr[1 % 6] = 122; cells[5] += arr[1 % 6];
    static int f3(int *p, int *q) { *p += 771; return *q * 2; }
    sum += f3(&cells[1], &cells[2]);
    cells[7] = cells[1] + 746;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    cells[4] = cells[2] + 696;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }