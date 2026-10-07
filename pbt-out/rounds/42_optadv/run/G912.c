static int f1(int *p, int *q) { *p += 815; return *q * 2; }
static int f2(int *p, int *q) { *p += 137; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 737; cells[5] += arr[3 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 286u; }
    cells[0] = cells[5] + 496;
    cells[3] = cells[5] + 303;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 891u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += f1(&cells[2], &cells[6]);
    cells[6] = cells[3] + 586;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[3] + 539;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f2(&cells[0], &cells[6]);
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[3] = cells[1] + 998;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }