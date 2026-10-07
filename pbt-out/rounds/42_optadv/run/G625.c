static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[0] + 392;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 890u; }
    arr[6 % 6] = 415; cells[2] += arr[6 % 6];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    static int f1(int *p, int *q) { *p += 296; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    cells[0] = cells[2] + 871;
    static int f2(int *p, int *q) { *p += 865; return *q * 2; }
    sum += f2(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f3(int *p, int *q) { *p += 794; return *q * 2; }
    sum += f3(&cells[6], &cells[2]);
    cells[0] = cells[2] + 168;
    static int f4(int *p, int *q) { *p += 896; return *q * 2; }
    sum += f4(&cells[2], &cells[1]);
    cells[4] = cells[0] + 221;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 363u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }