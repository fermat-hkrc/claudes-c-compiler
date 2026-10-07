static int f1(int *p, int *q) { *p += 141; return *q * 2; }
static int f2(int *p, int *q) { *p += 813; return *q * 2; }
static int f3(int *p, int *q) { *p += 702; return *q * 2; }
static int f4(int *p, int *q) { *p += 529; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += f1(&cells[3], &cells[5]);
    cells[0] = cells[4] + 516;
    sum += f2(&cells[1], &cells[0]);
    sum += f3(&cells[6], &cells[3]);
    cells[5] = cells[2] + 839;
    arr[5 % 6] = 937; cells[3] += arr[5 % 6];
    sum += f4(&cells[0], &cells[4]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    cells[7] = cells[2] + 333;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }