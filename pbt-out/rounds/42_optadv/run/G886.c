static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[1] + 158;
    cells[2] = cells[7] + 542;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[2 % 6] = 432; cells[6] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 965; return *q * 2; }
    sum += f1(&cells[1], &cells[4]);
    arr[5 % 6] = 101; cells[4] += arr[5 % 6];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    cells[0] = cells[0] + 80;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 686u; }
    cells[4] = cells[7] + 543;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 321u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[6] + 970;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }