static int f1(int *p, int *q) { *p += 159; return *q * 2; }
static int f2(int *p, int *q) { *p += 147; return *q * 2; }
static int f3(int *p, int *q) { *p += 542; return *q * 2; }
static int f4(int *p, int *q) { *p += 416; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[1], &cells[0]);
    arr[4 % 6] = 750; cells[7] += arr[4 % 6];
    cells[5] = cells[1] + 572;
    arr[2 % 6] = 849; cells[6] += arr[2 % 6];
    arr[6 % 6] = 293; cells[0] += arr[6 % 6];
    cells[1] = cells[5] + 966;
    sum += f2(&cells[4], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 869u; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[7 % 6] = 268; cells[2] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f3(&cells[1], &cells[6]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += f4(&cells[0], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }