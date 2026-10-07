static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[4 % 6] = 945; cells[4] += arr[4 % 6];
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    cells[1] = cells[3] + 172;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 886u; }
    static int f1(int *p, int *q) { *p += 454; return *q * 2; }
    sum += f1(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    static int f2(int *p, int *q) { *p += 400; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }