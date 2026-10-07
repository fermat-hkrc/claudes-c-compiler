static int f1(int *p, int *q) { *p += 329; return *q * 2; }
static int f2(int *p, int *q) { *p += 467; return *q * 2; }
static int f3(int *p, int *q) { *p += 878; return *q * 2; }
static int f4(int *p, int *q) { *p += 737; return *q * 2; }
static int f5(int *p, int *q) { *p += 280; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[1]);
    sum += f2(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[1 % 6] = 896; cells[4] += arr[1 % 6];
    sum += f3(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 845; cells[5] += arr[4 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[1], &cells[2]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[6 % 6] = 909; cells[2] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 128u; }
    sum += f5(&cells[6], &cells[4]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[5 % 6] = 299; cells[3] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[2] + 684;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }