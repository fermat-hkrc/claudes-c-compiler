static int f1(int *p, int *q) { *p += 332; return *q * 2; }
static int f2(int *p, int *q) { *p += 33; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[6 % 6] = 76; cells[7] += arr[6 % 6];
    sum += f1(&cells[0], &cells[5]);
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 568u; }
    cells[5] = cells[0] + 156;
    arr[2 % 6] = 928; cells[4] += arr[2 % 6];
    arr[1 % 6] = 752; cells[3] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f2(&cells[7], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }