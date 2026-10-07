static int f1(int *p, int *q) { *p += 980; return *q * 2; }
static int f2(int *p, int *q) { *p += 525; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[7 % 6] = 419; cells[2] += arr[7 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += f1(&cells[0], &cells[5]);
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 985u; }
    arr[7 % 6] = 728; cells[2] += arr[7 % 6];
    arr[3 % 6] = 828; cells[1] += arr[3 % 6];
    sum += f2(&cells[2], &cells[6]);
    cells[2] = cells[5] + 750;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 306u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 27u; }
    arr[3 % 6] = 258; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }