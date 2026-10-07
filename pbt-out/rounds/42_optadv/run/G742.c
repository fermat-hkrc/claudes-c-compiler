static int f1(int *p, int *q) { *p += 553; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[4] = cells[7] + 480;
    sum += f1(&cells[1], &cells[7]);
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 281u; }
    arr[3 % 6] = 627; cells[4] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }