static int f1(int *p, int *q) { *p += 521; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 802; cells[7] += arr[6 % 6];
    sum += f1(&cells[0], &cells[1]);
    arr[5 % 6] = 944; cells[0] += arr[5 % 6];
    arr[7 % 6] = 614; cells[4] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 530u; }
    arr[5 % 6] = 177; cells[5] += arr[5 % 6];
    cells[4] = cells[6] + 247;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 132u; }
    cells[3] = cells[7] + 764;
    arr[1 % 6] = 87; cells[5] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }