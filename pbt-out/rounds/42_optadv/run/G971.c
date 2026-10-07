static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    cells[1] = cells[3] + 597;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 664u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 892u; }
    static int f1(int *p, int *q) { *p += 356; return *q * 2; }
    sum += f1(&cells[3], &cells[7]);
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    arr[0 % 6] = 372; cells[7] += arr[0 % 6];
    static int f2(int *p, int *q) { *p += 105; return *q * 2; }
    sum += f2(&cells[2], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[3] + 104;
    arr[2 % 6] = 158; cells[2] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 915; return *q * 2; }
    sum += f3(&cells[5], &cells[0]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 379u; }
    cells[5] = cells[2] + 591;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    static int f4(int *p, int *q) { *p += 875; return *q * 2; }
    sum += f4(&cells[5], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }