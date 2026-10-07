static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[2 % 6] = 690; cells[7] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 72; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 306u; }
    static int f2(int *p, int *q) { *p += 537; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    static int f3(int *p, int *q) { *p += 91; return *q * 2; }
    sum += f3(&cells[5], &cells[6]);
    cells[1] = cells[2] + 276;
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[0 % 6] = 676; cells[3] += arr[0 % 6];
    cells[4] = cells[1] + 756;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 599u; }
    arr[3 % 6] = 126; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[4 % 6] = 506; cells[6] += arr[4 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    static int f4(int *p, int *q) { *p += 122; return *q * 2; }
    sum += f4(&cells[6], &cells[6]);
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 598u; }
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    cells[2] = cells[3] + 522;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }