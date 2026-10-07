static int f1(int *p, int *q) { *p += 972; return *q * 2; }
static int f2(int *p, int *q) { *p += 687; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[5] + 934;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[0] + 213;
    sum += f1(&cells[7], &cells[1]);
    arr[6 % 6] = 549; cells[0] += arr[6 % 6];
    sum += f2(&cells[7], &cells[6]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 608u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }