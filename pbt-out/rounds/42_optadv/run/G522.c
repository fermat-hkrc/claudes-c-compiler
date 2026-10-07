static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 517; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[4] = cells[1] + 973;
    arr[2 % 6] = 287; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[3] = cells[1] + 614;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 86u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 758u; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 887; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[0 % 6] = 730; cells[1] += arr[0 % 6];
    cells[5] = cells[1] + 928;
    arr[3 % 6] = 32; cells[1] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 271; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    cells[6] = cells[0] + 482;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }