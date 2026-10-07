static int f1(int *p, int *q) { *p += 908; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 844; cells[4] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    arr[6 % 6] = 360; cells[5] += arr[6 % 6];
    sum += f1(&cells[3], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }