static int f1(int *p, int *q) { *p += 503; return *q * 2; }
static int f2(int *p, int *q) { *p += 651; return *q * 2; }
static int f3(int *p, int *q) { *p += 132; return *q * 2; }
static int f4(int *p, int *q) { *p += 262; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 29; cells[6] += arr[3 % 6];
    arr[5 % 6] = 422; cells[0] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 464u; }
    sum += f1(&cells[3], &cells[4]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f2(&cells[5], &cells[2]);
    arr[5 % 6] = 418; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[3] = cells[2] + 423;
    cells[3] = cells[1] + 524;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[3], &cells[1]);
    sum += f4(&cells[7], &cells[1]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 46u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }