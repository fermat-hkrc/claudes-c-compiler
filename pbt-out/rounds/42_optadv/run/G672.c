static int f1(int *p, int *q) { *p += 115; return *q * 2; }
static int f2(int *p, int *q) { *p += 361; return *q * 2; }
static int f3(int *p, int *q) { *p += 119; return *q * 2; }
static int f4(int *p, int *q) { *p += 528; return *q * 2; }
static int f5(int *p, int *q) { *p += 985; return *q * 2; }
static int f6(int *p, int *q) { *p += 213; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[7 % 6] = 613; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f2(&cells[7], &cells[0]);
    sum += f3(&cells[5], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[1 % 6] = 79; cells[1] += arr[1 % 6];
    sum += f4(&cells[6], &cells[1]);
    cells[2] = cells[2] + 18;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 320u; }
    sum += f5(&cells[1], &cells[4]);
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[1] = cells[4] + 164;
    sum += f6(&cells[1], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 474; cells[1] += arr[5 % 6];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 710u; }
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }