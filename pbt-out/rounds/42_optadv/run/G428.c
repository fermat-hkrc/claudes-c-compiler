static int f1(int *p, int *q) { *p += 129; return *q * 2; }
static int f2(int *p, int *q) { *p += 688; return *q * 2; }
static int f3(int *p, int *q) { *p += 276; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[2] + 244;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 477u; }
    arr[6 % 6] = 204; cells[1] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 327; cells[4] += arr[3 % 6];
    sum += f1(&cells[3], &cells[3]);
    sum += f2(&cells[5], &cells[0]);
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[4], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }