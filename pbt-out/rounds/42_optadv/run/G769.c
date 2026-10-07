static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 255u; }
    arr[7 % 6] = 458; cells[3] += arr[7 % 6];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 271u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 195u; }
    cells[5] = cells[6] + 248;
    arr[0 % 6] = 698; cells[1] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 188u; }
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    static int f1(int *p, int *q) { *p += 479; return *q * 2; }
    sum += f1(&cells[0], &cells[5]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[6] = cells[5] + 400;
    static int f2(int *p, int *q) { *p += 139; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 137u; }
    arr[5 % 6] = 798; cells[7] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    static int f3(int *p, int *q) { *p += 615; return *q * 2; }
    sum += f3(&cells[7], &cells[0]);
    static int f4(int *p, int *q) { *p += 293; return *q * 2; }
    sum += f4(&cells[3], &cells[3]);
    static int f5(int *p, int *q) { *p += 939; return *q * 2; }
    sum += f5(&cells[3], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }