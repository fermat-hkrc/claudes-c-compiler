static int f1(int *p, int *q) { *p += 296; return *q * 2; }
static int f2(int *p, int *q) { *p += 801; return *q * 2; }
static int f3(int *p, int *q) { *p += 58; return *q * 2; }
static int f4(int *p, int *q) { *p += 181; return *q * 2; }
static int f5(int *p, int *q) { *p += 208; return *q * 2; }
static int f6(int *p, int *q) { *p += 814; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[1]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[2], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[7] = cells[3] + 145;
    arr[5 % 6] = 960; cells[1] += arr[5 % 6];
    sum += f3(&cells[4], &cells[5]);
    sum += f4(&cells[0], &cells[2]);
    sum += f5(&cells[1], &cells[4]);
    sum += f6(&cells[1], &cells[0]);
    arr[6 % 6] = 935; cells[7] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[6] = cells[7] + 544;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 179u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }