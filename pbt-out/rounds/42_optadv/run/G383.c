static int f1(int *p, int *q) { *p += 521; return *q * 2; }
static int f2(int *p, int *q) { *p += 290; return *q * 2; }
static int f3(int *p, int *q) { *p += 700; return *q * 2; }
static int f4(int *p, int *q) { *p += 58; return *q * 2; }
static int f5(int *p, int *q) { *p += 188; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 155u; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[6], &cells[1]);
    sum += f2(&cells[0], &cells[6]);
    sum += f3(&cells[2], &cells[2]);
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 849u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f4(&cells[1], &cells[2]);
    cells[6] = cells[5] + 49;
    cells[2] = cells[7] + 898;
    sum += f5(&cells[7], &cells[5]);
    cells[0] = cells[7] + 95;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 743u; }
    cells[3] = cells[7] + 603;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }