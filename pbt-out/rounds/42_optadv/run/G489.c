static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 122; return *q * 2; }
    sum += f1(&cells[4], &cells[4]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 166u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 155u; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 389u; }
    static int f2(int *p, int *q) { *p += 393; return *q * 2; }
    sum += f2(&cells[7], &cells[2]);
    static int f3(int *p, int *q) { *p += 540; return *q * 2; }
    sum += f3(&cells[1], &cells[7]);
    static int f4(int *p, int *q) { *p += 900; return *q * 2; }
    sum += f4(&cells[2], &cells[1]);
    cells[3] = cells[3] + 960;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[1] = cells[0] + 381;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 572u; }
    static int f5(int *p, int *q) { *p += 959; return *q * 2; }
    sum += f5(&cells[3], &cells[0]);
    cells[4] = cells[0] + 890;
    cells[1] = cells[1] + 291;
    arr[4 % 6] = 743; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f6(int *p, int *q) { *p += 176; return *q * 2; }
    sum += f6(&cells[5], &cells[7]);
    arr[2 % 6] = 236; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }