static int f1(int *p, int *q) { *p += 627; return *q * 2; }
static int f2(int *p, int *q) { *p += 657; return *q * 2; }
static int f3(int *p, int *q) { *p += 238; return *q * 2; }
static int f4(int *p, int *q) { *p += 729; return *q * 2; }
static int f5(int *p, int *q) { *p += 338; return *q * 2; }
static int f6(int *p, int *q) { *p += 560; return *q * 2; }
static int f7(int *p, int *q) { *p += 280; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[6], &cells[4]);
    cells[7] = cells[7] + 657;
    arr[4 % 6] = 220; cells[0] += arr[4 % 6];
    sum += f3(&cells[3], &cells[2]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[1 % 6] = 178; cells[6] += arr[1 % 6];
    sum += f4(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f5(&cells[7], &cells[3]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 728u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 10u; }
    sum += f6(&cells[7], &cells[3]);
    sum += f7(&cells[7], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }