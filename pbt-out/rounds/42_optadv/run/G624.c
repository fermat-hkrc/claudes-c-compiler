static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 837u; }
    static int f1(int *p, int *q) { *p += 593; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[6 % 6] = 132; cells[6] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 303; return *q * 2; }
    sum += f2(&cells[7], &cells[6]);
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 165u; }
    arr[5 % 6] = 474; cells[0] += arr[5 % 6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }