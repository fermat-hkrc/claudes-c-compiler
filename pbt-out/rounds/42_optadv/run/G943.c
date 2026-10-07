static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 167; cells[3] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 658; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 940u; }
    static int f2(int *p, int *q) { *p += 855; return *q * 2; }
    sum += f2(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 934u; }
    arr[0 % 6] = 124; cells[5] += arr[0 % 6];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[3 % 6] = 753; cells[2] += arr[3 % 6];
    arr[7 % 6] = 934; cells[1] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f3(int *p, int *q) { *p += 957; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    cells[0] = cells[1] + 269;
    arr[3 % 6] = 567; cells[0] += arr[3 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[0 % 6] = 475; cells[6] += arr[0 % 6];
    cells[3] = cells[5] + 566;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }