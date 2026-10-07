static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    static int f1(int *p, int *q) { *p += 606; return *q * 2; }
    sum += f1(&cells[1], &cells[5]);
    cells[7] = cells[6] + 76;
    arr[6 % 6] = 570; cells[2] += arr[6 % 6];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 531u; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 911u; }
    static int f2(int *p, int *q) { *p += 436; return *q * 2; }
    sum += f2(&cells[2], &cells[4]);
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[5 % 6] = 772; cells[7] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 863u; }
    arr[2 % 6] = 386; cells[3] += arr[2 % 6];
    cells[5] = cells[5] + 78;
    cells[7] = cells[3] + 249;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 363u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }