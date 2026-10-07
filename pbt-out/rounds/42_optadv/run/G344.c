static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 628u; }
    cells[1] = cells[6] + 447;
    static int f1(int *p, int *q) { *p += 631; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    arr[7 % 6] = 335; cells[5] += arr[7 % 6];
    static int f2(int *p, int *q) { *p += 503; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 497u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 467u; }
    static int f3(int *p, int *q) { *p += 210; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    static int f4(int *p, int *q) { *p += 843; return *q * 2; }
    sum += f4(&cells[0], &cells[2]);
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }