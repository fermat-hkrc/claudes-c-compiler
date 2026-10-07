static int f1(int *p, int *q) { *p += 562; return *q * 2; }
static int f2(int *p, int *q) { *p += 507; return *q * 2; }
static int f3(int *p, int *q) { *p += 972; return *q * 2; }
static int f4(int *p, int *q) { *p += 654; return *q * 2; }
static int f5(int *p, int *q) { *p += 377; return *q * 2; }
static int f6(int *p, int *q) { *p += 25; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 612; cells[1] += arr[6 % 6];
    sum += f1(&cells[5], &cells[6]);
    sum += f2(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f3(&cells[4], &cells[3]);
    sum += f4(&cells[7], &cells[1]);
    sum += f5(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[0] + 691;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 852u; }
    sum += f6(&cells[4], &cells[0]);
    arr[2 % 6] = 313; cells[5] += arr[2 % 6];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 729u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }