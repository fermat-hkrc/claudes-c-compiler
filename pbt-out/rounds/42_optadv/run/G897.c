static int f1(int *p, int *q) { *p += 300; return *q * 2; }
static int f2(int *p, int *q) { *p += 866; return *q * 2; }
static int f3(int *p, int *q) { *p += 829; return *q * 2; }
static int f4(int *p, int *q) { *p += 436; return *q * 2; }
static int f5(int *p, int *q) { *p += 623; return *q * 2; }
static int f6(int *p, int *q) { *p += 956; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[1] + 272;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 643u; }
    arr[6 % 6] = 972; cells[7] += arr[6 % 6];
    cells[3] = cells[7] + 722;
    sum += f1(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[3], &cells[7]);
    arr[4 % 6] = 680; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    arr[5 % 6] = 218; cells[3] += arr[5 % 6];
    sum += f3(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    arr[2 % 6] = 656; cells[1] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 769u; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    arr[2 % 6] = 647; cells[6] += arr[2 % 6];
    sum += f4(&cells[6], &cells[6]);
    cells[5] = cells[4] + 291;
    sum += f5(&cells[1], &cells[0]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f6(&cells[2], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }