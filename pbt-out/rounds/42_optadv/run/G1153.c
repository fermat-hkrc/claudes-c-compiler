static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[7] = cells[0] + 49;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 387u; }
    cells[0] = cells[2] + 387;
    arr[2 % 6] = 254; cells[2] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 495; return *q * 2; }
    sum += f1(&cells[0], &cells[0]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 360u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f2(int *p, int *q) { *p += 922; return *q * 2; }
    sum += f2(&cells[5], &cells[7]);
    cells[4] = cells[4] + 772;
    cells[6] = cells[7] + 600;
    static int f3(int *p, int *q) { *p += 255; return *q * 2; }
    sum += f3(&cells[1], &cells[1]);
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 567u; }
    arr[3 % 6] = 874; cells[3] += arr[3 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    arr[7 % 6] = 178; cells[2] += arr[7 % 6];
    static int f4(int *p, int *q) { *p += 317; return *q * 2; }
    sum += f4(&cells[7], &cells[6]);
    arr[0 % 6] = 982; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    static int f5(int *p, int *q) { *p += 108; return *q * 2; }
    sum += f5(&cells[7], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }