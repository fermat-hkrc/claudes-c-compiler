static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 748u; }
    arr[1 % 6] = 625; cells[3] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[6 % 6] = 749; cells[0] += arr[6 % 6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 109u; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[6] = cells[0] + 709;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 624u; }
    static int f1(int *p, int *q) { *p += 187; return *q * 2; }
    sum += f1(&cells[4], &cells[2]);
    static int f2(int *p, int *q) { *p += 129; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    arr[4 % 6] = 883; cells[6] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 607u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }