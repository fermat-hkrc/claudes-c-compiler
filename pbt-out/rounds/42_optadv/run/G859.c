static int f1(int *p, int *q) { *p += 681; return *q * 2; }
static int f2(int *p, int *q) { *p += 567; return *q * 2; }
static int f3(int *p, int *q) { *p += 213; return *q * 2; }
static int f4(int *p, int *q) { *p += 881; return *q * 2; }
static int f5(int *p, int *q) { *p += 532; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[2]);
    sum += f2(&cells[0], &cells[2]);
    cells[4] = cells[6] + 690;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 80u; }
    sum += f3(&cells[2], &cells[7]);
    cells[0] = cells[1] + 154;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += f4(&cells[3], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[2] = cells[0] + 832;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += f5(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    arr[5 % 6] = 217; cells[6] += arr[5 % 6];
    arr[6 % 6] = 142; cells[5] += arr[6 % 6];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 127u; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[4 % 6] = 606; cells[3] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }