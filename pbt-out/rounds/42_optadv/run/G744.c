static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[4] = cells[6] + 920;
    cells[0] = cells[4] + 269;
    static int f1(int *p, int *q) { *p += 864; return *q * 2; }
    sum += f1(&cells[0], &cells[1]);
    cells[4] = cells[7] + 79;
    cells[0] = cells[0] + 581;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 486u; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    static int f2(int *p, int *q) { *p += 281; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    static int f3(int *p, int *q) { *p += 425; return *q * 2; }
    sum += f3(&cells[1], &cells[0]);
    static int f4(int *p, int *q) { *p += 955; return *q * 2; }
    sum += f4(&cells[0], &cells[5]);
    cells[7] = cells[2] + 610;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f5(int *p, int *q) { *p += 401; return *q * 2; }
    sum += f5(&cells[5], &cells[3]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 313u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }