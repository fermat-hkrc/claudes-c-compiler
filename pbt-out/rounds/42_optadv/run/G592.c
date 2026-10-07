static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[3] + 475;
    static int f1(int *p, int *q) { *p += 959; return *q * 2; }
    sum += f1(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[3] = cells[0] + 196;
    static int f2(int *p, int *q) { *p += 706; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    cells[2] = cells[0] + 410;
    arr[7 % 6] = 271; cells[3] += arr[7 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 164u; }
    cells[6] = cells[4] + 95;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 550u; }
    static int f3(int *p, int *q) { *p += 505; return *q * 2; }
    sum += f3(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[0 % 6] = 859; cells[6] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 300u; }
    arr[3 % 6] = 704; cells[0] += arr[3 % 6];
    cells[3] = cells[1] + 243;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }