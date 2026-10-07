static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    static int f1(int *p, int *q) { *p += 106; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    static int f2(int *p, int *q) { *p += 561; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 735u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[0] = cells[4] + 471;
    arr[3 % 6] = 16; cells[3] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f3(int *p, int *q) { *p += 656; return *q * 2; }
    sum += f3(&cells[4], &cells[6]);
    arr[7 % 6] = 938; cells[1] += arr[7 % 6];
    arr[0 % 6] = 72; cells[7] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 71u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }