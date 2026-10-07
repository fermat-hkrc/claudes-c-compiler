static int f1(int *p, int *q) { *p += 712; return *q * 2; }
static int f2(int *p, int *q) { *p += 500; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[1]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 558u; }
    arr[0 % 6] = 550; cells[0] += arr[0 % 6];
    cells[6] = cells[7] + 831;
    cells[2] = cells[1] + 812;
    cells[0] = cells[0] + 298;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 778u; }
    sum += f2(&cells[1], &cells[7]);
    arr[1 % 6] = 239; cells[1] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }