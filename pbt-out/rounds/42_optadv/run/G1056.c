static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 73; cells[5] += arr[7 % 6];
    arr[3 % 6] = 481; cells[7] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 113; return *q * 2; }
    sum += f1(&cells[3], &cells[1]);
    cells[0] = cells[0] + 499;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    static int f2(int *p, int *q) { *p += 704; return *q * 2; }
    sum += f2(&cells[2], &cells[4]);
    cells[3] = cells[0] + 284;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 968u; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f3(int *p, int *q) { *p += 938; return *q * 2; }
    sum += f3(&cells[2], &cells[0]);
    arr[4 % 6] = 757; cells[2] += arr[4 % 6];
    cells[4] = cells[2] + 896;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }