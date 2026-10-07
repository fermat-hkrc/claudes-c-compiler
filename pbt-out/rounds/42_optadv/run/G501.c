static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 171; cells[2] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 156u; }
    arr[3 % 6] = 677; cells[5] += arr[3 % 6];
    cells[7] = cells[4] + 60;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[4] = cells[3] + 731;
    cells[1] = cells[2] + 276;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 124u; }
    cells[2] = cells[0] + 629;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 438u; }
    static int f1(int *p, int *q) { *p += 801; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    static int f2(int *p, int *q) { *p += 186; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    cells[3] = cells[7] + 37;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    static int f3(int *p, int *q) { *p += 452; return *q * 2; }
    sum += f3(&cells[1], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }