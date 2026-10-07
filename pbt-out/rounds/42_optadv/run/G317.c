static int f1(int *p, int *q) { *p += 899; return *q * 2; }
static int f2(int *p, int *q) { *p += 309; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 973u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[6] + 437;
    arr[2 % 6] = 664; cells[0] += arr[2 % 6];
    sum += f1(&cells[6], &cells[4]);
    cells[2] = cells[0] + 958;
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    cells[3] = cells[3] + 28;
    arr[7 % 6] = 824; cells[7] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }