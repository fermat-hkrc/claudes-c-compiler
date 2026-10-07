static int f1(int *p, int *q) { *p += 104; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[6] = cells[5] + 515;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 83u; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 634u; }
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    arr[4 % 6] = 870; cells[5] += arr[4 % 6];
    arr[6 % 6] = 872; cells[5] += arr[6 % 6];
    sum += f1(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[5] + 519;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }