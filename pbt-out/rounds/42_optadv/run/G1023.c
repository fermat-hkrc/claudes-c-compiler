static int f1(int *p, int *q) { *p += 976; return *q * 2; }
static int f2(int *p, int *q) { *p += 22; return *q * 2; }
static int f3(int *p, int *q) { *p += 414; return *q * 2; }
static int f4(int *p, int *q) { *p += 404; return *q * 2; }
static int f5(int *p, int *q) { *p += 207; return *q * 2; }
static int f6(int *p, int *q) { *p += 931; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 162;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[0], &cells[1]);
    sum += f2(&cells[3], &cells[7]);
    cells[1] = cells[0] + 12;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 578u; }
    cells[1] = cells[3] + 964;
    sum += f3(&cells[7], &cells[6]);
    sum += f4(&cells[3], &cells[3]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 187u; }
    sum += f5(&cells[2], &cells[3]);
    cells[0] = cells[3] + 376;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f6(&cells[7], &cells[1]);
    arr[2 % 6] = 105; cells[7] += arr[2 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 6u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }