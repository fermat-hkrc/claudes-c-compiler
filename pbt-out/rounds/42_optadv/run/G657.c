static int f1(int *p, int *q) { *p += 394; return *q * 2; }
static int f2(int *p, int *q) { *p += 241; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 318; cells[6] += arr[4 % 6];
    cells[4] = cells[7] + 480;
    sum += f1(&cells[0], &cells[3]);
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 74u; }
    arr[4 % 6] = 871; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 397; cells[4] += arr[4 % 6];
    sum += f2(&cells[4], &cells[3]);
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 612u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }