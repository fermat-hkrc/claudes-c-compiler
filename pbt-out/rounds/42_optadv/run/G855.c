static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 376; cells[4] += arr[0 % 6];
    cells[3] = cells[5] + 727;
    arr[7 % 6] = 411; cells[2] += arr[7 % 6];
    arr[5 % 6] = 58; cells[2] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    static int f1(int *p, int *q) { *p += 315; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f2(int *p, int *q) { *p += 523; return *q * 2; }
    sum += f2(&cells[1], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 389u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }