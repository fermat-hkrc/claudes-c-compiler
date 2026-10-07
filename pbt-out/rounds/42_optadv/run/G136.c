static int f1(int *p, int *q) { *p += 889; return *q * 2; }
static int f2(int *p, int *q) { *p += 892; return *q * 2; }
static int f3(int *p, int *q) { *p += 724; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[6] + 872;
    sum += f1(&cells[5], &cells[7]);
    arr[2 % 6] = 481; cells[4] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f2(&cells[4], &cells[7]);
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 400u; }
    sum += f3(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 580; cells[3] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }