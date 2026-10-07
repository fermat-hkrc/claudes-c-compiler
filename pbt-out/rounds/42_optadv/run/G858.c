static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 527; cells[4] += arr[5 % 6];
    cells[3] = cells[7] + 450;
    static int f1(int *p, int *q) { *p += 902; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    static int f2(int *p, int *q) { *p += 244; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    static int f3(int *p, int *q) { *p += 518; return *q * 2; }
    sum += f3(&cells[0], &cells[6]);
    arr[0 % 6] = 143; cells[7] += arr[0 % 6];
    static int f4(int *p, int *q) { *p += 559; return *q * 2; }
    sum += f4(&cells[4], &cells[1]);
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    static int f5(int *p, int *q) { *p += 372; return *q * 2; }
    sum += f5(&cells[7], &cells[7]);
    arr[4 % 6] = 204; cells[4] += arr[4 % 6];
    arr[7 % 6] = 898; cells[3] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 377u; }
    arr[3 % 6] = 962; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }