static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 155u; }
    static int f1(int *p, int *q) { *p += 877; return *q * 2; }
    sum += f1(&cells[3], &cells[7]);
    static int f2(int *p, int *q) { *p += 192; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    arr[2 % 6] = 720; cells[1] += arr[2 % 6];
    arr[0 % 6] = 326; cells[7] += arr[0 % 6];
    cells[3] = cells[1] + 662;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    static int f3(int *p, int *q) { *p += 127; return *q * 2; }
    sum += f3(&cells[0], &cells[7]);
    cells[3] = cells[7] + 404;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[6] = cells[1] + 882;
    cells[6] = cells[4] + 606;
    cells[5] = cells[5] + 348;
    arr[3 % 6] = 782; cells[5] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 85u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }