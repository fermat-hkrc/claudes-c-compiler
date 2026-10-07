static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 488; cells[4] += arr[4 % 6];
    arr[2 % 6] = 812; cells[2] += arr[2 % 6];
    cells[1] = cells[6] + 129;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[0 % 6] = 45; cells[0] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 595u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[5 % 6] = 28; cells[3] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 513u; }
    static int f1(int *p, int *q) { *p += 594; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    cells[5] = cells[1] + 26;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 932u; }
    cells[3] = cells[7] + 727;
    static int f2(int *p, int *q) { *p += 704; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }