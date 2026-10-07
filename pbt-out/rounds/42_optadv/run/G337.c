static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 79u; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    cells[5] = cells[0] + 787;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 428u; }
    arr[0 % 6] = 754; cells[3] += arr[0 % 6];
    static int f1(int *p, int *q) { *p += 165; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 861u; }
    cells[0] = cells[1] + 662;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }