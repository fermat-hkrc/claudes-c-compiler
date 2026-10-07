static int f1(int *p, int *q) { *p += 46; return *q * 2; }
static int f2(int *p, int *q) { *p += 226; return *q * 2; }
static int f3(int *p, int *q) { *p += 854; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += f1(&cells[1], &cells[5]);
    arr[6 % 6] = 973; cells[7] += arr[6 % 6];
    sum += f2(&cells[3], &cells[2]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += f3(&cells[0], &cells[2]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    arr[5 % 6] = 711; cells[3] += arr[5 % 6];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }