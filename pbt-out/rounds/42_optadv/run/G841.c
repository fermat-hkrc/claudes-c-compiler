static int f1(int *p, int *q) { *p += 718; return *q * 2; }
static int f2(int *p, int *q) { *p += 165; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 663; cells[0] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 146u; }
    arr[0 % 6] = 594; cells[0] += arr[0 % 6];
    sum += f1(&cells[4], &cells[5]);
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 653u; }
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    cells[7] = cells[4] + 357;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 639u; }
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }