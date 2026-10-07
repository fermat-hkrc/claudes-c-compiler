static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[1] + 111;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    static int f1(int *p, int *q) { *p += 213; return *q * 2; }
    sum += f1(&cells[2], &cells[2]);
    static int f2(int *p, int *q) { *p += 426; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    cells[7] = cells[0] + 906;
    static int f3(int *p, int *q) { *p += 807; return *q * 2; }
    sum += f3(&cells[4], &cells[4]);
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 254u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 917u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f4(int *p, int *q) { *p += 299; return *q * 2; }
    sum += f4(&cells[7], &cells[7]);
    static int f5(int *p, int *q) { *p += 330; return *q * 2; }
    sum += f5(&cells[6], &cells[7]);
    arr[4 % 6] = 263; cells[3] += arr[4 % 6];
    cells[7] = cells[3] + 304;
    static int f6(int *p, int *q) { *p += 342; return *q * 2; }
    sum += f6(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 753u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }