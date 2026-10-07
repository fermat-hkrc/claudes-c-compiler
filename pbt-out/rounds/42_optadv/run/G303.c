static int f1(int *p, int *q) { *p += 677; return *q * 2; }
static int f2(int *p, int *q) { *p += 153; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 183; cells[3] += arr[0 % 6];
    cells[6] = cells[3] + 760;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[3] + 598;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f1(&cells[3], &cells[2]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 847u; }
    sum += f2(&cells[6], &cells[4]);
    arr[5 % 6] = 798; cells[2] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 301u; }
    arr[2 % 6] = 36; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }