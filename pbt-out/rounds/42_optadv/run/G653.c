static int f1(int *p, int *q) { *p += 963; return *q * 2; }
static int f2(int *p, int *q) { *p += 881; return *q * 2; }
static int f3(int *p, int *q) { *p += 325; return *q * 2; }
static int f4(int *p, int *q) { *p += 410; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 586; cells[0] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    arr[0 % 6] = 562; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[2], &cells[0]);
    sum += f2(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[1], &cells[1]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    cells[2] = cells[4] + 32;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 435u; }
    cells[5] = cells[4] + 566;
    cells[2] = cells[5] + 693;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 25u; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[5] + 237;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f4(&cells[7], &cells[1]);
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    arr[5 % 6] = 54; cells[4] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }