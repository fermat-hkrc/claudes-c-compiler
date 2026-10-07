static int f1(int *p, int *q) { *p += 37; return *q * 2; }
static int f2(int *p, int *q) { *p += 149; return *q * 2; }
static int f3(int *p, int *q) { *p += 302; return *q * 2; }
static int f4(int *p, int *q) { *p += 635; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[0]);
    sum += f2(&cells[1], &cells[7]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 879u; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 74u; }
    sum += f3(&cells[4], &cells[4]);
    cells[4] = cells[5] + 878;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 606u; }
    arr[7 % 6] = 956; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 587; cells[4] += arr[1 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 84u; }
    sum += f4(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }