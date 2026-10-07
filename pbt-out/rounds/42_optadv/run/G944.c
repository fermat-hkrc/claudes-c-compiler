static int f1(int *p, int *q) { *p += 696; return *q * 2; }
static int f2(int *p, int *q) { *p += 970; return *q * 2; }
static int f3(int *p, int *q) { *p += 54; return *q * 2; }
static int f4(int *p, int *q) { *p += 573; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += f1(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 653u; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 904u; }
    sum += f2(&cells[3], &cells[6]);
    arr[2 % 6] = 864; cells[0] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[2 % 6] = 958; cells[2] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[6] = cells[4] + 406;
    arr[3 % 6] = 759; cells[1] += arr[3 % 6];
    cells[4] = cells[2] + 467;
    arr[4 % 6] = 162; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += f3(&cells[6], &cells[1]);
    arr[7 % 6] = 999; cells[4] += arr[7 % 6];
    sum += f4(&cells[1], &cells[3]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }