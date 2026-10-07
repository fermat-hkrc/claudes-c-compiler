static int f1(int *p, int *q) { *p += 430; return *q * 2; }
static int f2(int *p, int *q) { *p += 677; return *q * 2; }
static int f3(int *p, int *q) { *p += 32; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[6]);
    arr[1 % 6] = 210; cells[3] += arr[1 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[3 % 6] = 405; cells[3] += arr[3 % 6];
    cells[3] = cells[0] + 141;
    arr[5 % 6] = 134; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 980u; }
    cells[3] = cells[2] + 398;
    arr[6 % 6] = 74; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[6] = cells[3] + 298;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 375u; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 530u; }
    sum += f2(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[3] = cells[4] + 801;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[1] + 218;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 620u; }
    sum += f3(&cells[7], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }