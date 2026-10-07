static int f1(int *p, int *q) { *p += 240; return *q * 2; }
static int f2(int *p, int *q) { *p += 288; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[3 % 6] = 419; cells[3] += arr[3 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += f1(&cells[3], &cells[3]);
    cells[0] = cells[3] + 754;
    arr[3 % 6] = 817; cells[4] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 487u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += f2(&cells[0], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 857u; }
    arr[6 % 6] = 220; cells[1] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }