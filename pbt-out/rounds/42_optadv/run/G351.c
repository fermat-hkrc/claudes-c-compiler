static int f1(int *p, int *q) { *p += 556; return *q * 2; }
static int f2(int *p, int *q) { *p += 397; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 607u; }
    cells[2] = cells[1] + 796;
    cells[3] = cells[1] + 284;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 899u; }
    sum += f1(&cells[2], &cells[5]);
    sum += f2(&cells[5], &cells[4]);
    arr[3 % 6] = 662; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 223u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 360u; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[7] = cells[6] + 100;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    cells[1] = cells[0] + 560;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }