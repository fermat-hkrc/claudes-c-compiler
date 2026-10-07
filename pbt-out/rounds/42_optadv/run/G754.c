static int f1(int *p, int *q) { *p += 639; return *q * 2; }
static int f2(int *p, int *q) { *p += 119; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[1] = cells[6] + 930;
    sum += f1(&cells[7], &cells[6]);
    arr[0 % 6] = 734; cells[3] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 564u; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    arr[0 % 6] = 827; cells[1] += arr[0 % 6];
    sum += f2(&cells[4], &cells[4]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 970u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }