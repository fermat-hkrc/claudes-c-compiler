static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 192; return *q * 2; }
    sum += f1(&cells[2], &cells[1]);
    cells[4] = cells[4] + 854;
    arr[1 % 6] = 883; cells[7] += arr[1 % 6];
    arr[7 % 6] = 628; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[4] = cells[2] + 25;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 150u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 225u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    arr[7 % 6] = 181; cells[5] += arr[7 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 41u; }
    static int f2(int *p, int *q) { *p += 559; return *q * 2; }
    sum += f2(&cells[6], &cells[6]);
    static int f3(int *p, int *q) { *p += 90; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    arr[5 % 6] = 381; cells[3] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }