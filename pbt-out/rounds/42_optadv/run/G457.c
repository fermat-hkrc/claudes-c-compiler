static int f1(int *p, int *q) { *p += 992; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 644u; }
    sum += f1(&cells[5], &cells[2]);
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[6] = cells[6] + 90;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[4] = cells[3] + 638;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 602u; }
    cells[1] = cells[2] + 138;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[5 % 6] = 328; cells[4] += arr[5 % 6];
    cells[2] = cells[4] + 983;
    cells[0] = cells[0] + 774;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 773u; }
    arr[7 % 6] = 544; cells[3] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 854; cells[6] += arr[6 % 6];
    arr[2 % 6] = 203; cells[2] += arr[2 % 6];
    arr[7 % 6] = 148; cells[4] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }