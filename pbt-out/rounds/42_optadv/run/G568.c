static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 823; cells[0] += arr[0 % 6];
    cells[3] = cells[3] + 420;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f1(int *p, int *q) { *p += 498; return *q * 2; }
    sum += f1(&cells[4], &cells[0]);
    static int f2(int *p, int *q) { *p += 149; return *q * 2; }
    sum += f2(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[0] = cells[0] + 787;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f3(int *p, int *q) { *p += 445; return *q * 2; }
    sum += f3(&cells[0], &cells[3]);
    static int f4(int *p, int *q) { *p += 890; return *q * 2; }
    sum += f4(&cells[5], &cells[3]);
    cells[2] = cells[1] + 64;
    cells[5] = cells[5] + 827;
    arr[7 % 6] = 983; cells[0] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 157u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }