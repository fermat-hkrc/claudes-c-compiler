static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[2] = cells[3] + 795;
    arr[1 % 6] = 507; cells[7] += arr[1 % 6];
    cells[1] = cells[2] + 412;
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 111u; }
    cells[1] = cells[1] + 394;
    cells[1] = cells[1] + 26;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f1(int *p, int *q) { *p += 271; return *q * 2; }
    sum += f1(&cells[7], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }