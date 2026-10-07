static int f1(int *p, int *q) { *p += 282; return *q * 2; }
static int f2(int *p, int *q) { *p += 604; return *q * 2; }
static int f3(int *p, int *q) { *p += 696; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[4] + 829;
    sum += f1(&cells[3], &cells[1]);
    sum += f2(&cells[4], &cells[6]);
    cells[1] = cells[4] + 623;
    arr[0 % 6] = 690; cells[5] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f3(&cells[2], &cells[7]);
    cells[5] = cells[6] + 384;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 785u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }