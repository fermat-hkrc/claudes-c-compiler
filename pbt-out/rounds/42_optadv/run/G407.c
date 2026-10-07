static int f1(int *p, int *q) { *p += 965; return *q * 2; }
static int f2(int *p, int *q) { *p += 759; return *q * 2; }
static int f3(int *p, int *q) { *p += 102; return *q * 2; }
static int f4(int *p, int *q) { *p += 879; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    cells[5] = cells[5] + 489;
    arr[2 % 6] = 983; cells[4] += arr[2 % 6];
    cells[4] = cells[5] + 892;
    cells[7] = cells[4] + 10;
    sum += f1(&cells[6], &cells[7]);
    arr[0 % 6] = 744; cells[4] += arr[0 % 6];
    cells[7] = cells[1] + 503;
    sum += f2(&cells[2], &cells[2]);
    sum += f3(&cells[4], &cells[6]);
    sum += f4(&cells[5], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 264u; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    cells[1] = cells[4] + 416;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 273u; }
    arr[3 % 6] = 145; cells[6] += arr[3 % 6];
    cells[4] = cells[7] + 767;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 665u; }
    cells[0] = cells[5] + 742;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }