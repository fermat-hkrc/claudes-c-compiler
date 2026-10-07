static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    static int f1(int *p, int *q) { *p += 880; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    static int f2(int *p, int *q) { *p += 530; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    cells[3] = cells[4] + 56;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 4u; }
    static int f3(int *p, int *q) { *p += 767; return *q * 2; }
    sum += f3(&cells[6], &cells[1]);
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 449u; }
    static int f4(int *p, int *q) { *p += 169; return *q * 2; }
    sum += f4(&cells[6], &cells[6]);
    cells[3] = cells[1] + 476;
    arr[1 % 6] = 563; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }