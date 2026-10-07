static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 577; return *q * 2; }
    sum += f1(&cells[5], &cells[1]);
    arr[3 % 6] = 905; cells[0] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 892; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    arr[2 % 6] = 599; cells[1] += arr[2 % 6];
    arr[6 % 6] = 295; cells[6] += arr[6 % 6];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[2 % 6] = 963; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }