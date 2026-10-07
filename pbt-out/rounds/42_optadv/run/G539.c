static int f1(int *p, int *q) { *p += 671; return *q * 2; }
static int f2(int *p, int *q) { *p += 66; return *q * 2; }
static int f3(int *p, int *q) { *p += 174; return *q * 2; }
static int f4(int *p, int *q) { *p += 578; return *q * 2; }
static int f5(int *p, int *q) { *p += 907; return *q * 2; }
static int f6(int *p, int *q) { *p += 690; return *q * 2; }
static int f7(int *p, int *q) { *p += 431; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[0]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 83u; }
    sum += f2(&cells[6], &cells[2]);
    sum += f3(&cells[5], &cells[6]);
    arr[0 % 6] = 611; cells[2] += arr[0 % 6];
    sum += f4(&cells[0], &cells[1]);
    cells[6] = cells[7] + 829;
    cells[2] = cells[1] + 239;
    sum += f5(&cells[4], &cells[3]);
    cells[1] = cells[5] + 174;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[4] = cells[2] + 869;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 744u; }
    arr[6 % 6] = 162; cells[1] += arr[6 % 6];
    sum += f6(&cells[5], &cells[5]);
    sum += f7(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }