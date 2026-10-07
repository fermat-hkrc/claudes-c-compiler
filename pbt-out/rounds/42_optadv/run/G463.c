static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 483; cells[3] += arr[7 % 6];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[3] + 11;
    static int f1(int *p, int *q) { *p += 445; return *q * 2; }
    sum += f1(&cells[4], &cells[0]);
    arr[3 % 6] = 733; cells[3] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 98; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 837u; }
    arr[2 % 6] = 204; cells[0] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 447; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 695u; }
    cells[1] = cells[7] + 376;
    cells[6] = cells[6] + 181;
    cells[3] = cells[3] + 172;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 260u; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }