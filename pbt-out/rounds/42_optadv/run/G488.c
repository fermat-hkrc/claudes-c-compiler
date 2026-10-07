static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 688; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    arr[2 % 6] = 743; cells[0] += arr[2 % 6];
    cells[4] = cells[5] + 61;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 198; cells[4] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 787u; }
    cells[1] = cells[7] + 639;
    cells[2] = cells[2] + 664;
    static int f2(int *p, int *q) { *p += 304; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    static int f3(int *p, int *q) { *p += 104; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }