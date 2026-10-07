static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 153; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    static int f2(int *p, int *q) { *p += 4; return *q * 2; }
    sum += f2(&cells[4], &cells[2]);
    cells[2] = cells[5] + 997;
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 873u; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    static int f3(int *p, int *q) { *p += 20; return *q * 2; }
    sum += f3(&cells[0], &cells[3]);
    arr[6 % 6] = 460; cells[2] += arr[6 % 6];
    cells[0] = cells[0] + 972;
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 43u; }
    cells[5] = cells[6] + 665;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 453u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f4(int *p, int *q) { *p += 169; return *q * 2; }
    sum += f4(&cells[1], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[4] = cells[0] + 762;
    static int f5(int *p, int *q) { *p += 661; return *q * 2; }
    sum += f5(&cells[1], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }