static int f1(int *p, int *q) { *p += 456; return *q * 2; }
static int f2(int *p, int *q) { *p += 691; return *q * 2; }
static int f3(int *p, int *q) { *p += 945; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 321u; }
    arr[2 % 6] = 829; cells[0] += arr[2 % 6];
    arr[1 % 6] = 307; cells[5] += arr[1 % 6];
    arr[4 % 6] = 747; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[0], &cells[3]);
    cells[5] = cells[5] + 270;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 395u; }
    sum += f3(&cells[2], &cells[1]);
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 909u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }