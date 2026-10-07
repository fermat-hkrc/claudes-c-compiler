static int f1(int *p, int *q) { *p += 327; return *q * 2; }
static int f2(int *p, int *q) { *p += 792; return *q * 2; }
static int f3(int *p, int *q) { *p += 901; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[5 % 6] = 770; cells[5] += arr[5 % 6];
    cells[1] = cells[0] + 32;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f1(&cells[1], &cells[7]);
    cells[7] = cells[0] + 778;
    arr[7 % 6] = 237; cells[4] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 595u; }
    sum += f2(&cells[2], &cells[6]);
    sum += f3(&cells[6], &cells[6]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 906u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 532u; }
    arr[3 % 6] = 540; cells[3] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[0 % 6] = 121; cells[6] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }