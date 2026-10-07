static int f1(int *p, int *q) { *p += 282; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 235; cells[1] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 51u; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 794u; }
    cells[7] = cells[1] + 91;
    sum += f1(&cells[5], &cells[5]);
    cells[5] = cells[5] + 740;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }