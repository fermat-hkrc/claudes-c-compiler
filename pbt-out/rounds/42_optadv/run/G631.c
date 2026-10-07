static int f1(int *p, int *q) { *p += 328; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 435; cells[4] += arr[4 % 6];
    sum += f1(&cells[2], &cells[4]);
    cells[1] = cells[5] + 100;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 753u; }
    cells[0] = cells[5] + 595;
    arr[7 % 6] = 154; cells[5] += arr[7 % 6];
    arr[2 % 6] = 345; cells[3] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 779u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }