static int f1(int *p, int *q) { *p += 917; return *q * 2; }
static int f2(int *p, int *q) { *p += 425; return *q * 2; }
static int f3(int *p, int *q) { *p += 677; return *q * 2; }
static int f4(int *p, int *q) { *p += 623; return *q * 2; }
static int f5(int *p, int *q) { *p += 629; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[3] + 455;
    sum += f1(&cells[0], &cells[3]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 112u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[1] = cells[2] + 224;
    cells[1] = cells[3] + 496;
    sum += f2(&cells[0], &cells[7]);
    cells[4] = cells[7] + 266;
    sum += f3(&cells[4], &cells[1]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 353u; }
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[3 % 6] = 988; cells[2] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 522u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 821u; }
    sum += f4(&cells[7], &cells[7]);
    sum += f5(&cells[3], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }