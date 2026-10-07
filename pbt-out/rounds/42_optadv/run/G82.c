static int f1(int *p, int *q) { *p += 633; return *q * 2; }
static int f2(int *p, int *q) { *p += 500; return *q * 2; }
static int f3(int *p, int *q) { *p += 935; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[7]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 627; cells[6] += arr[0 % 6];
    sum += f2(&cells[3], &cells[7]);
    sum += f3(&cells[4], &cells[7]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 195u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[6] = cells[5] + 437;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[2 % 6] = 885; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }