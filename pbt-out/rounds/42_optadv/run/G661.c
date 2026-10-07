static int f1(int *p, int *q) { *p += 954; return *q * 2; }
static int f2(int *p, int *q) { *p += 424; return *q * 2; }
static int f3(int *p, int *q) { *p += 414; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += f1(&cells[0], &cells[2]);
    sum += f2(&cells[5], &cells[4]);
    arr[1 % 6] = 847; cells[7] += arr[1 % 6];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 636u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[0], &cells[7]);
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[6 % 6] = 254; cells[5] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }