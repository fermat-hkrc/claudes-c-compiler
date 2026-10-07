static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 471; return *q * 2; }
    sum += f1(&cells[2], &cells[6]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 514u; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[0 % 6] = 875; cells[0] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 974u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }