static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 285u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    static int f1(int *p, int *q) { *p += 237; return *q * 2; }
    sum += f1(&cells[4], &cells[5]);
    static int f2(int *p, int *q) { *p += 431; return *q * 2; }
    sum += f2(&cells[3], &cells[1]);
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 249u; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    static int f3(int *p, int *q) { *p += 2; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f4(int *p, int *q) { *p += 666; return *q * 2; }
    sum += f4(&cells[7], &cells[7]);
    static int f5(int *p, int *q) { *p += 586; return *q * 2; }
    sum += f5(&cells[2], &cells[3]);
    arr[1 % 6] = 68; cells[6] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[7] = cells[6] + 585;
    cells[2] = cells[0] + 877;
    static int f6(int *p, int *q) { *p += 642; return *q * 2; }
    sum += f6(&cells[4], &cells[3]);
    arr[1 % 6] = 888; cells[1] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }