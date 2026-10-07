static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 142u; }
    static int f1(int *p, int *q) { *p += 317; return *q * 2; }
    sum += f1(&cells[5], &cells[3]);
    arr[6 % 6] = 54; cells[0] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    static int f2(int *p, int *q) { *p += 736; return *q * 2; }
    sum += f2(&cells[1], &cells[4]);
    arr[0 % 6] = 354; cells[1] += arr[0 % 6];
    cells[6] = cells[7] + 549;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 756u; }
    static int f3(int *p, int *q) { *p += 760; return *q * 2; }
    sum += f3(&cells[3], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }