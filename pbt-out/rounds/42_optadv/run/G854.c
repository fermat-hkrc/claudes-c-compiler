static int f1(int *p, int *q) { *p += 960; return *q * 2; }
static int f2(int *p, int *q) { *p += 802; return *q * 2; }
static int f3(int *p, int *q) { *p += 418; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 96; cells[3] += arr[7 % 6];
    sum += f1(&cells[7], &cells[0]);
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 103; cells[5] += arr[5 % 6];
    arr[0 % 6] = 954; cells[4] += arr[0 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 939u; }
    arr[5 % 6] = 615; cells[7] += arr[5 % 6];
    sum += f2(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 930; cells[5] += arr[6 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f3(&cells[4], &cells[7]);
    arr[1 % 6] = 258; cells[6] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }