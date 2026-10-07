static int f1(int *p, int *q) { *p += 534; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 500; cells[2] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[7] = cells[3] + 186;
    sum += f1(&cells[3], &cells[4]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 215u; }
    cells[3] = cells[4] + 13;
    arr[3 % 6] = 958; cells[6] += arr[3 % 6];
    arr[5 % 6] = 457; cells[3] += arr[5 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    arr[3 % 6] = 812; cells[3] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }