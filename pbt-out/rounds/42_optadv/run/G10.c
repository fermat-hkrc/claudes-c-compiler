static int f1(int *p, int *q) { *p += 800; return *q * 2; }
static int f2(int *p, int *q) { *p += 963; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[2] + 309;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 341u; }
    arr[4 % 6] = 314; cells[0] += arr[4 % 6];
    arr[5 % 6] = 503; cells[1] += arr[5 % 6];
    sum += f1(&cells[7], &cells[2]);
    arr[7 % 6] = 710; cells[6] += arr[7 % 6];
    cells[5] = cells[5] + 593;
    cells[5] = cells[0] + 425;
    sum += f2(&cells[3], &cells[4]);
    arr[3 % 6] = 54; cells[2] += arr[3 % 6];
    arr[1 % 6] = 699; cells[1] += arr[1 % 6];
    arr[4 % 6] = 352; cells[0] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 21u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }