static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f1(int *p, int *q) { *p += 468; return *q * 2; }
    sum += f1(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f2(int *p, int *q) { *p += 314; return *q * 2; }
    sum += f2(&cells[7], &cells[3]);
    static int f3(int *p, int *q) { *p += 711; return *q * 2; }
    sum += f3(&cells[3], &cells[4]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 955u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[1 % 6] = 307; cells[5] += arr[1 % 6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 751u; }
    arr[1 % 6] = 242; cells[2] += arr[1 % 6];
    static int f4(int *p, int *q) { *p += 4; return *q * 2; }
    sum += f4(&cells[3], &cells[5]);
    static int f5(int *p, int *q) { *p += 470; return *q * 2; }
    sum += f5(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }