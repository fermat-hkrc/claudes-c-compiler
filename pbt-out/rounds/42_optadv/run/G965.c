static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 332; return *q * 2; }
    sum += f1(&cells[6], &cells[4]);
    arr[0 % 6] = 616; cells[1] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 863u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[1 % 6] = 544; cells[5] += arr[1 % 6];
    cells[1] = cells[0] + 209;
    arr[1 % 6] = 878; cells[3] += arr[1 % 6];
    cells[3] = cells[5] + 95;
    arr[0 % 6] = 290; cells[2] += arr[0 % 6];
    cells[0] = cells[1] + 716;
    cells[4] = cells[2] + 970;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }