static int f1(int *p, int *q) { *p += 576; return *q * 2; }
static int f2(int *p, int *q) { *p += 732; return *q * 2; }
static int f3(int *p, int *q) { *p += 270; return *q * 2; }
static int f4(int *p, int *q) { *p += 43; return *q * 2; }
static int f5(int *p, int *q) { *p += 882; return *q * 2; }
static int f6(int *p, int *q) { *p += 799; return *q * 2; }
static int f7(int *p, int *q) { *p += 66; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[1], &cells[5]);
    sum += f2(&cells[0], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f3(&cells[4], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 822u; }
    sum += f4(&cells[0], &cells[5]);
    sum += f5(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f6(&cells[4], &cells[5]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 213u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 578u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += f7(&cells[1], &cells[4]);
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 629u; }
    cells[0] = cells[6] + 727;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }