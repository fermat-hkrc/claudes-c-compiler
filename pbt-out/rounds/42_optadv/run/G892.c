static int f1(int *p, int *q) { *p += 298; return *q * 2; }
static int f2(int *p, int *q) { *p += 380; return *q * 2; }
static int f3(int *p, int *q) { *p += 336; return *q * 2; }
static int f4(int *p, int *q) { *p += 739; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    arr[1 % 6] = 84; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 289u; }
    arr[6 % 6] = 117; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[7], &cells[2]);
    cells[3] = cells[1] + 691;
    sum += f2(&cells[3], &cells[7]);
    arr[3 % 6] = 213; cells[5] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[4] + 31;
    sum += f3(&cells[4], &cells[2]);
    sum += f4(&cells[5], &cells[1]);
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 304u; }
    arr[3 % 6] = 806; cells[3] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[7 % 6] = 643; cells[2] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }