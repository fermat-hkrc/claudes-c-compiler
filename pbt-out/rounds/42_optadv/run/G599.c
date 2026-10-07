static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 830; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 32u; }
    static int f2(int *p, int *q) { *p += 57; return *q * 2; }
    sum += f2(&cells[1], &cells[3]);
    static int f3(int *p, int *q) { *p += 388; return *q * 2; }
    sum += f3(&cells[0], &cells[2]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 782u; }
    arr[2 % 6] = 410; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }