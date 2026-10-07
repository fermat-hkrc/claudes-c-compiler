static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 668; return *q * 2; }
    sum += f1(&cells[5], &cells[3]);
    arr[7 % 6] = 561; cells[2] += arr[7 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 398u; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 559u; }
    cells[3] = cells[6] + 100;
    arr[4 % 6] = 142; cells[3] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 896; return *q * 2; }
    sum += f2(&cells[1], &cells[6]);
    static int f3(int *p, int *q) { *p += 473; return *q * 2; }
    sum += f3(&cells[1], &cells[3]);
    cells[7] = cells[6] + 61;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[4 % 6] = 472; cells[5] += arr[4 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 960u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }