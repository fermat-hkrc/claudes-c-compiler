static int f1(int *p, int *q) { *p += 770; return *q * 2; }
static int f2(int *p, int *q) { *p += 638; return *q * 2; }
static int f3(int *p, int *q) { *p += 736; return *q * 2; }
static int f4(int *p, int *q) { *p += 106; return *q * 2; }
static int f5(int *p, int *q) { *p += 647; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[2] + 488;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[2], &cells[0]);
    sum += f2(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[3] = cells[2] + 37;
    sum += f3(&cells[4], &cells[6]);
    sum += f4(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[2 % 6] = 420; cells[0] += arr[2 % 6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 663u; }
    sum += f5(&cells[6], &cells[2]);
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 44u; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[7] = cells[2] + 799;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }