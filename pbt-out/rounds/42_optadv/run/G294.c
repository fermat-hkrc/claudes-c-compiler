static int f1(int *p, int *q) { *p += 282; return *q * 2; }
static int f2(int *p, int *q) { *p += 228; return *q * 2; }
static int f3(int *p, int *q) { *p += 930; return *q * 2; }
static int f4(int *p, int *q) { *p += 645; return *q * 2; }
static int f5(int *p, int *q) { *p += 157; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[2] = cells[1] + 624;
    sum += f1(&cells[3], &cells[3]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    cells[7] = cells[6] + 736;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 384; cells[0] += arr[4 % 6];
    arr[2 % 6] = 548; cells[4] += arr[2 % 6];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[3 % 6] = 219; cells[4] += arr[3 % 6];
    sum += f2(&cells[1], &cells[5]);
    sum += f3(&cells[5], &cells[4]);
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 608u; }
    sum += f4(&cells[4], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[7] = cells[7] + 1;
    sum += f5(&cells[6], &cells[3]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 298u; }
    arr[1 % 6] = 927; cells[5] += arr[1 % 6];
    cells[2] = cells[7] + 789;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }