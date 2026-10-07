static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 857; return *q * 2; }
    sum += f1(&cells[4], &cells[3]);
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[1] = cells[7] + 582;
    static int f2(int *p, int *q) { *p += 250; return *q * 2; }
    sum += f2(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[2] + 30;
    static int f3(int *p, int *q) { *p += 838; return *q * 2; }
    sum += f3(&cells[5], &cells[3]);
    static int f4(int *p, int *q) { *p += 830; return *q * 2; }
    sum += f4(&cells[0], &cells[3]);
    cells[5] = cells[7] + 564;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 154u; }
    arr[4 % 6] = 105; cells[0] += arr[4 % 6];
    cells[5] = cells[4] + 230;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }