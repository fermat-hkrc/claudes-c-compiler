static int f1(int *p, int *q) { *p += 629; return *q * 2; }
static int f2(int *p, int *q) { *p += 459; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[1 % 6] = 462; cells[4] += arr[1 % 6];
    sum += f1(&cells[3], &cells[5]);
    arr[4 % 6] = 849; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    arr[0 % 6] = 523; cells[3] += arr[0 % 6];
    cells[4] = cells[4] + 979;
    arr[2 % 6] = 40; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 549u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 123u; }
    cells[5] = cells[5] + 171;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 17u; }
    cells[2] = cells[7] + 719;
    arr[5 % 6] = 599; cells[1] += arr[5 % 6];
    sum += f2(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }