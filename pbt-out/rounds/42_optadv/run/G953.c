static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 453; return *q * 2; }
    sum += f1(&cells[3], &cells[7]);
    static int f2(int *p, int *q) { *p += 732; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 878; cells[3] += arr[1 % 6];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 517u; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[4 % 6] = 247; cells[6] += arr[4 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 877u; }
    cells[0] = cells[5] + 112;
    arr[1 % 6] = 152; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 380; cells[4] += arr[1 % 6];
    arr[3 % 6] = 537; cells[5] += arr[3 % 6];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[6] = cells[6] + 941;
    cells[3] = cells[5] + 893;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 176; cells[1] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }