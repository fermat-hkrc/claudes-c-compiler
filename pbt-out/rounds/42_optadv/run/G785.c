static int f1(int *p, int *q) { *p += 56; return *q * 2; }
static int f2(int *p, int *q) { *p += 37; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 532; cells[5] += arr[4 % 6];
    sum += f1(&cells[2], &cells[5]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 883u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[1] = cells[6] + 707;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[7] = cells[3] + 446;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f2(&cells[6], &cells[0]);
    cells[2] = cells[3] + 745;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 114u; }
    arr[3 % 6] = 638; cells[2] += arr[3 % 6];
    cells[1] = cells[2] + 691;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 588u; }
    arr[4 % 6] = 941; cells[3] += arr[4 % 6];
    arr[4 % 6] = 461; cells[3] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 857u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }