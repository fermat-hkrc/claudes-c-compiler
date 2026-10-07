static int f1(int *p, int *q) { *p += 167; return *q * 2; }
static int f2(int *p, int *q) { *p += 195; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 22; cells[1] += arr[1 % 6];
    cells[5] = cells[0] + 891;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 912u; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += f1(&cells[5], &cells[0]);
    cells[2] = cells[5] + 329;
    sum += f2(&cells[5], &cells[5]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 137u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }