static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 385; cells[1] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 970; return *q * 2; }
    sum += f1(&cells[0], &cells[0]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    static int f2(int *p, int *q) { *p += 957; return *q * 2; }
    sum += f2(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f3(int *p, int *q) { *p += 73; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    cells[6] = cells[1] + 999;
    arr[1 % 6] = 291; cells[3] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }