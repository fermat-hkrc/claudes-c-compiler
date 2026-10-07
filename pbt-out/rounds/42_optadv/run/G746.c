static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 419; cells[4] += arr[5 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[3 % 6] = 344; cells[7] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 227; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    static int f2(int *p, int *q) { *p += 459; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    arr[3 % 6] = 179; cells[2] += arr[3 % 6];
    cells[5] = cells[3] + 748;
    cells[1] = cells[2] + 856;
    cells[5] = cells[3] + 433;
    arr[0 % 6] = 218; cells[5] += arr[0 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 784u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }