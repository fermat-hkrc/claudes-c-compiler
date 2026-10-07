static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 681; cells[3] += arr[5 % 6];
    arr[5 % 6] = 639; cells[0] += arr[5 % 6];
    cells[2] = cells[7] + 529;
    static int f1(int *p, int *q) { *p += 924; return *q * 2; }
    sum += f1(&cells[3], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f2(int *p, int *q) { *p += 580; return *q * 2; }
    sum += f2(&cells[2], &cells[7]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 697u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }