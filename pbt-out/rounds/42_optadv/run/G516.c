static int f1(int *p, int *q) { *p += 71; return *q * 2; }
static int f2(int *p, int *q) { *p += 6; return *q * 2; }
static int f3(int *p, int *q) { *p += 59; return *q * 2; }
static int f4(int *p, int *q) { *p += 539; return *q * 2; }
static int f5(int *p, int *q) { *p += 466; return *q * 2; }
static int f6(int *p, int *q) { *p += 250; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 781; cells[3] += arr[1 % 6];
    arr[5 % 6] = 723; cells[6] += arr[5 % 6];
    sum += f1(&cells[6], &cells[1]);
    sum += f2(&cells[3], &cells[1]);
    arr[2 % 6] = 621; cells[2] += arr[2 % 6];
    cells[7] = cells[5] + 735;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 568u; }
    arr[0 % 6] = 378; cells[6] += arr[0 % 6];
    sum += f3(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 480u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[0] = cells[2] + 962;
    sum += f4(&cells[6], &cells[3]);
    sum += f5(&cells[6], &cells[4]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 421u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    sum += f6(&cells[4], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }