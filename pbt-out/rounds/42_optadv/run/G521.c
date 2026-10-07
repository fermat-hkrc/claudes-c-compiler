static int f1(int *p, int *q) { *p += 224; return *q * 2; }
static int f2(int *p, int *q) { *p += 782; return *q * 2; }
static int f3(int *p, int *q) { *p += 763; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[0] + 584;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 751u; }
    cells[5] = cells[4] + 781;
    sum += f1(&cells[3], &cells[2]);
    sum += f2(&cells[3], &cells[6]);
    sum += f3(&cells[4], &cells[2]);
    arr[2 % 6] = 661; cells[7] += arr[2 % 6];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 706u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }