static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 131u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f1(int *p, int *q) { *p += 749; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    static int f2(int *p, int *q) { *p += 27; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[7] = cells[3] + 209;
    cells[2] = cells[1] + 632;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }