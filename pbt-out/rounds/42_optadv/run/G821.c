static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 529; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[7] + 120;
    arr[2 % 6] = 767; cells[5] += arr[2 % 6];
    cells[1] = cells[7] + 826;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    static int f2(int *p, int *q) { *p += 446; return *q * 2; }
    sum += f2(&cells[4], &cells[0]);
    cells[7] = cells[2] + 490;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f3(int *p, int *q) { *p += 491; return *q * 2; }
    sum += f3(&cells[2], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    static int f4(int *p, int *q) { *p += 1; return *q * 2; }
    sum += f4(&cells[2], &cells[3]);
    arr[3 % 6] = 49; cells[5] += arr[3 % 6];
    arr[3 % 6] = 87; cells[1] += arr[3 % 6];
    arr[5 % 6] = 976; cells[3] += arr[5 % 6];
    cells[6] = cells[7] + 349;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 397u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }