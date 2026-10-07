static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 251; cells[1] += arr[4 % 6];
    arr[3 % 6] = 599; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 371; return *q * 2; }
    sum += f1(&cells[4], &cells[7]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f2(int *p, int *q) { *p += 931; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[2 % 6] = 428; cells[3] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 10u; }
    static int f3(int *p, int *q) { *p += 43; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    arr[5 % 6] = 114; cells[7] += arr[5 % 6];
    arr[6 % 6] = 181; cells[3] += arr[6 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[6] = cells[7] + 437;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }