static int f1(int *p, int *q) { *p += 488; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[2]);
    arr[4 % 6] = 479; cells[1] += arr[4 % 6];
    cells[5] = cells[6] + 330;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 681u; }
    cells[0] = cells[1] + 866;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 269u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }