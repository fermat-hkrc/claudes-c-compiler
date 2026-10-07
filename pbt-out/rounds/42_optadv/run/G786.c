static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 634u; }
    static int f1(int *p, int *q) { *p += 616; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 502u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    arr[3 % 6] = 815; cells[0] += arr[3 % 6];
    cells[0] = cells[0] + 724;
    arr[6 % 6] = 147; cells[1] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 745; return *q * 2; }
    sum += f2(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f3(int *p, int *q) { *p += 187; return *q * 2; }
    sum += f3(&cells[6], &cells[7]);
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 716u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }