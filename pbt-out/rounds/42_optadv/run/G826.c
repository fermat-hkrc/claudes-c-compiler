static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 931; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 848; cells[0] += arr[1 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    static int f1(int *p, int *q) { *p += 308; return *q * 2; }
    sum += f1(&cells[4], &cells[7]);
    arr[4 % 6] = 300; cells[1] += arr[4 % 6];
    cells[4] = cells[5] + 325;
    static int f2(int *p, int *q) { *p += 871; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }