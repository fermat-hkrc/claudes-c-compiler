static int f1(int *p, int *q) { *p += 199; return *q * 2; }
static int f2(int *p, int *q) { *p += 939; return *q * 2; }
static int f3(int *p, int *q) { *p += 782; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[3]);
    sum += f2(&cells[4], &cells[6]);
    arr[5 % 6] = 50; cells[1] += arr[5 % 6];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 121u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 934; cells[1] += arr[2 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[7] = cells[0] + 757;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f3(&cells[5], &cells[2]);
    cells[2] = cells[5] + 677;
    cells[7] = cells[5] + 153;
    arr[4 % 6] = 226; cells[4] += arr[4 % 6];
    arr[3 % 6] = 828; cells[7] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }