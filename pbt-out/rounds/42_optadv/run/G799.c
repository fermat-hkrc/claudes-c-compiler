static int f1(int *p, int *q) { *p += 374; return *q * 2; }
static int f2(int *p, int *q) { *p += 912; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 16u; }
    cells[5] = cells[7] + 619;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += f1(&cells[3], &cells[1]);
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 733u; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[5] = cells[2] + 172;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f2(&cells[3], &cells[2]);
    cells[2] = cells[7] + 936;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 438u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 825u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }