static int f1(int *p, int *q) { *p += 426; return *q * 2; }
static int f2(int *p, int *q) { *p += 134; return *q * 2; }
static int f3(int *p, int *q) { *p += 395; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[6] + 364;
    sum += f1(&cells[6], &cells[7]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 488u; }
    cells[0] = cells[3] + 30;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 318u; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[7], &cells[0]);
    arr[4 % 6] = 498; cells[0] += arr[4 % 6];
    sum += f3(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 486; cells[0] += arr[3 % 6];
    arr[2 % 6] = 818; cells[6] += arr[2 % 6];
    arr[4 % 6] = 900; cells[3] += arr[4 % 6];
    cells[7] = cells[2] + 938;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }