static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 633; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    static int f1(int *p, int *q) { *p += 115; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    static int f2(int *p, int *q) { *p += 283; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[4 % 6] = 484; cells[2] += arr[4 % 6];
    arr[7 % 6] = 270; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 141u; }
    cells[1] = cells[7] + 595;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[3] = cells[1] + 793;
    static int f3(int *p, int *q) { *p += 660; return *q * 2; }
    sum += f3(&cells[6], &cells[3]);
    arr[3 % 6] = 519; cells[5] += arr[3 % 6];
    static int f4(int *p, int *q) { *p += 964; return *q * 2; }
    sum += f4(&cells[2], &cells[7]);
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 499u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }