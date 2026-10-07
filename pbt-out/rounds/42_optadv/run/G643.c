static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[4] + 169;
    static int f1(int *p, int *q) { *p += 297; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    cells[3] = cells[1] + 8;
    static int f2(int *p, int *q) { *p += 918; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    cells[1] = cells[4] + 35;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 687u; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 18u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[5] = cells[5] + 691;
    static int f3(int *p, int *q) { *p += 380; return *q * 2; }
    sum += f3(&cells[4], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[5 % 6] = 925; cells[5] += arr[5 % 6];
    arr[3 % 6] = 917; cells[4] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 733u; }
    static int f4(int *p, int *q) { *p += 112; return *q * 2; }
    sum += f4(&cells[6], &cells[7]);
    static int f5(int *p, int *q) { *p += 369; return *q * 2; }
    sum += f5(&cells[1], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }