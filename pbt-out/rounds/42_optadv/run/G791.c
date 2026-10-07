static int f1(int *p, int *q) { *p += 699; return *q * 2; }
static int f2(int *p, int *q) { *p += 384; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[4] + 199;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 6u; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 603u; }
    sum += f1(&cells[6], &cells[5]);
    arr[0 % 6] = 878; cells[7] += arr[0 % 6];
    cells[1] = cells[6] + 207;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[0] = cells[5] + 742;
    sum += f2(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }