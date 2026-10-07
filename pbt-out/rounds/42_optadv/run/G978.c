static int f1(int *p, int *q) { *p += 863; return *q * 2; }
static int f2(int *p, int *q) { *p += 779; return *q * 2; }
static int f3(int *p, int *q) { *p += 916; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[6 % 6] = 691; cells[7] += arr[6 % 6];
    arr[5 % 6] = 162; cells[1] += arr[5 % 6];
    arr[2 % 6] = 968; cells[6] += arr[2 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[3] = cells[7] + 597;
    cells[3] = cells[3] + 970;
    sum += f1(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 30u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[5 % 6] = 964; cells[4] += arr[5 % 6];
    arr[4 % 6] = 164; cells[0] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    sum += f3(&cells[6], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }