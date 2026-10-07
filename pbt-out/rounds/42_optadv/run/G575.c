static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    static int f1(int *p, int *q) { *p += 406; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 364u; }
    arr[4 % 6] = 608; cells[7] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 547u; }
    static int f2(int *p, int *q) { *p += 426; return *q * 2; }
    sum += f2(&cells[4], &cells[1]);
    cells[0] = cells[2] + 694;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 648u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }