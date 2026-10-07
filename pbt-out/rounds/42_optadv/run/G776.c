static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    static int f1(int *p, int *q) { *p += 982; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    static int f2(int *p, int *q) { *p += 980; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    cells[4] = cells[4] + 247;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[0] = cells[6] + 561;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 731u; }
    cells[6] = cells[2] + 189;
    arr[0 % 6] = 726; cells[3] += arr[0 % 6];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 963u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 763u; }
    static int f3(int *p, int *q) { *p += 823; return *q * 2; }
    sum += f3(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[6] = cells[1] + 733;
    cells[1] = cells[4] + 326;
    static int f4(int *p, int *q) { *p += 545; return *q * 2; }
    sum += f4(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }