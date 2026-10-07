static int f1(int *p, int *q) { *p += 12; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[0] + 43;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[1 % 6] = 871; cells[1] += arr[1 % 6];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    cells[7] = cells[3] + 784;
    arr[5 % 6] = 328; cells[3] += arr[5 % 6];
    arr[4 % 6] = 143; cells[4] += arr[4 % 6];
    sum += f1(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }