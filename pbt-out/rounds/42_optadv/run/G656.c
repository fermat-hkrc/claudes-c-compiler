static int f1(int *p, int *q) { *p += 352; return *q * 2; }
static int f2(int *p, int *q) { *p += 712; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 695; cells[6] += arr[5 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 601u; }
    sum += f1(&cells[4], &cells[3]);
    arr[5 % 6] = 598; cells[2] += arr[5 % 6];
    sum += f2(&cells[4], &cells[5]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 382u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }