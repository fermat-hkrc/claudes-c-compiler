static int f1(int *p, int *q) { *p += 687; return *q * 2; }
static int f2(int *p, int *q) { *p += 511; return *q * 2; }
static int f3(int *p, int *q) { *p += 709; return *q * 2; }
static int f4(int *p, int *q) { *p += 454; return *q * 2; }
static int f5(int *p, int *q) { *p += 912; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f1(&cells[7], &cells[1]);
    sum += f2(&cells[0], &cells[2]);
    cells[7] = cells[0] + 868;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 346u; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 177u; }
    sum += f3(&cells[2], &cells[2]);
    sum += f4(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    cells[5] = cells[6] + 632;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 934u; }
    arr[3 % 6] = 802; cells[0] += arr[3 % 6];
    sum += f5(&cells[7], &cells[0]);
    cells[7] = cells[0] + 107;
    arr[2 % 6] = 790; cells[1] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 565u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }