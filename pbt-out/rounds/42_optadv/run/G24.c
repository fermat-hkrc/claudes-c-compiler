static int f1(int *p, int *q) { *p += 102; return *q * 2; }
static int f2(int *p, int *q) { *p += 106; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 609; cells[1] += arr[6 % 6];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += f1(&cells[1], &cells[4]);
    sum += f2(&cells[4], &cells[1]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 406u; }
    arr[0 % 6] = 141; cells[4] += arr[0 % 6];
    cells[3] = cells[0] + 161;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }