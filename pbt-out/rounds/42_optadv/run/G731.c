static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 187; return *q * 2; }
    sum += f1(&cells[3], &cells[1]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 295u; }
    arr[6 % 6] = 352; cells[1] += arr[6 % 6];
    cells[7] = cells[2] + 573;
    static int f2(int *p, int *q) { *p += 19; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[1 % 6] = 166; cells[3] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }