static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 468; cells[0] += arr[0 % 6];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[6 % 6] = 715; cells[1] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 213; return *q * 2; }
    sum += f1(&cells[1], &cells[4]);
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 175u; }
    static int f2(int *p, int *q) { *p += 128; return *q * 2; }
    sum += f2(&cells[1], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }