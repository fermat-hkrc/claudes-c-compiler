static int f1(int *p, int *q) { *p += 250; return *q * 2; }
static int f2(int *p, int *q) { *p += 201; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[3], &cells[6]);
    sum += f2(&cells[7], &cells[4]);
    cells[5] = cells[7] + 157;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 782u; }
    arr[4 % 6] = 423; cells[5] += arr[4 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }