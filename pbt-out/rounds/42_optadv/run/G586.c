static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 63; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    arr[7 % 6] = 291; cells[2] += arr[7 % 6];
    static int f2(int *p, int *q) { *p += 687; return *q * 2; }
    sum += f2(&cells[2], &cells[1]);
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 143u; }
    static int f3(int *p, int *q) { *p += 868; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 380u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 668; cells[3] += arr[0 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }