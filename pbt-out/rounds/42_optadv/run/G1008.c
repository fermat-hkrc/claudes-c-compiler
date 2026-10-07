static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[7] + 761;
    arr[1 % 6] = 42; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[7] = cells[3] + 659;
    static int f1(int *p, int *q) { *p += 973; return *q * 2; }
    sum += f1(&cells[7], &cells[5]);
    cells[6] = cells[6] + 756;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 164u; }
    arr[4 % 6] = 620; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 37; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    static int f3(int *p, int *q) { *p += 502; return *q * 2; }
    sum += f3(&cells[6], &cells[1]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[5] + 232;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 662u; }
    arr[3 % 6] = 980; cells[5] += arr[3 % 6];
    cells[5] = cells[6] + 494;
    static int f4(int *p, int *q) { *p += 116; return *q * 2; }
    sum += f4(&cells[1], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }