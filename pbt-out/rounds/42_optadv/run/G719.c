static int f1(int *p, int *q) { *p += 379; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 15; cells[7] += arr[0 % 6];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 212u; }
    cells[3] = cells[4] + 427;
    arr[4 % 6] = 4; cells[2] += arr[4 % 6];
    sum += f1(&cells[5], &cells[2]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    cells[5] = cells[4] + 968;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 524u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 466u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 764; cells[3] += arr[4 % 6];
    arr[7 % 6] = 697; cells[5] += arr[7 % 6];
    cells[3] = cells[5] + 597;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }