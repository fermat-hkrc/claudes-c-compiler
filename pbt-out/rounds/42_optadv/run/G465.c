static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 458u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 506u; }
    arr[5 % 6] = 863; cells[3] += arr[5 % 6];
    arr[6 % 6] = 344; cells[0] += arr[6 % 6];
    cells[1] = cells[0] + 274;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 832u; }
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    static int f1(int *p, int *q) { *p += 116; return *q * 2; }
    sum += f1(&cells[0], &cells[6]);
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 579u; }
    arr[2 % 6] = 198; cells[3] += arr[2 % 6];
    cells[4] = cells[6] + 322;
    arr[7 % 6] = 344; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[4 % 6] = 665; cells[1] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 573; return *q * 2; }
    sum += f2(&cells[3], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }