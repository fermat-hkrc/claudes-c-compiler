static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 496u; }
    arr[1 % 6] = 375; cells[2] += arr[1 % 6];
    cells[7] = cells[2] + 986;
    cells[0] = cells[7] + 847;
    arr[7 % 6] = 376; cells[3] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[7 % 6] = 438; cells[0] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 129; return *q * 2; }
    sum += f1(&cells[5], &cells[3]);
    arr[7 % 6] = 910; cells[2] += arr[7 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    cells[6] = cells[3] + 77;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 346u; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 85u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 714; cells[5] += arr[0 % 6];
    arr[1 % 6] = 792; cells[5] += arr[1 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 395u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }