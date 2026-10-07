static int f1(int *p, int *q) { *p += 330; return *q * 2; }
static int f2(int *p, int *q) { *p += 989; return *q * 2; }
static int f3(int *p, int *q) { *p += 101; return *q * 2; }
static int f4(int *p, int *q) { *p += 120; return *q * 2; }
static int f5(int *p, int *q) { *p += 603; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[6 % 6] = 487; cells[1] += arr[6 % 6];
    sum += f1(&cells[6], &cells[7]);
    arr[2 % 6] = 882; cells[2] += arr[2 % 6];
    arr[7 % 6] = 264; cells[5] += arr[7 % 6];
    arr[3 % 6] = 585; cells[4] += arr[3 % 6];
    arr[6 % 6] = 375; cells[0] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    cells[3] = cells[4] + 738;
    sum += f2(&cells[1], &cells[1]);
    arr[1 % 6] = 836; cells[1] += arr[1 % 6];
    cells[5] = cells[7] + 214;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += f3(&cells[2], &cells[7]);
    sum += f4(&cells[0], &cells[6]);
    sum += f5(&cells[7], &cells[5]);
    arr[3 % 6] = 673; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 96u; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }