static int f1(int *p, int *q) { *p += 525; return *q * 2; }
static int f2(int *p, int *q) { *p += 102; return *q * 2; }
static int f3(int *p, int *q) { *p += 128; return *q * 2; }
static int f4(int *p, int *q) { *p += 56; return *q * 2; }
static int f5(int *p, int *q) { *p += 29; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[7]);
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += f2(&cells[3], &cells[7]);
    sum += f3(&cells[7], &cells[1]);
    sum += f4(&cells[1], &cells[3]);
    arr[6 % 6] = 504; cells[6] += arr[6 % 6];
    cells[4] = cells[7] + 611;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    arr[5 % 6] = 525; cells[1] += arr[5 % 6];
    arr[7 % 6] = 115; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 47u; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f5(&cells[1], &cells[1]);
    cells[2] = cells[5] + 693;
    cells[4] = cells[6] + 522;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }