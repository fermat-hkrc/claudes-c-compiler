static int f1(int *p, int *q) { *p += 526; return *q * 2; }
static int f2(int *p, int *q) { *p += 394; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 422u; }
    arr[0 % 6] = 878; cells[4] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f1(&cells[2], &cells[5]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[0] = cells[2] + 522;
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[1] + 672;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[5 % 6] = 641; cells[2] += arr[5 % 6];
    arr[4 % 6] = 865; cells[5] += arr[4 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[0], &cells[6]);
    cells[6] = cells[6] + 665;
    cells[2] = cells[6] + 591;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }