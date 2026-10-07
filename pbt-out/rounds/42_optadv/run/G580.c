static int f1(int *p, int *q) { *p += 894; return *q * 2; }
static int f2(int *p, int *q) { *p += 35; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[0 % 6] = 19; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f1(&cells[5], &cells[7]);
    cells[0] = cells[6] + 657;
    sum += f2(&cells[1], &cells[1]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 134u; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 102u; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 576u; }
    arr[6 % 6] = 946; cells[1] += arr[6 % 6];
    cells[6] = cells[5] + 128;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }