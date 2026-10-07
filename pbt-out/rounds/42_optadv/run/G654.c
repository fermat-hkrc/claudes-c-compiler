static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 582; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    arr[6 % 6] = 557; cells[4] += arr[6 % 6];
    arr[6 % 6] = 140; cells[5] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 423; return *q * 2; }
    sum += f2(&cells[0], &cells[4]);
    cells[3] = cells[1] + 190;
    static int f3(int *p, int *q) { *p += 335; return *q * 2; }
    sum += f3(&cells[0], &cells[1]);
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 995u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[1] = cells[5] + 794;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }