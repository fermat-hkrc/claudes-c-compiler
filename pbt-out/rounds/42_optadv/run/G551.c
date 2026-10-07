static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 289u; }
    static int f1(int *p, int *q) { *p += 318; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[1] = cells[7] + 333;
    arr[2 % 6] = 268; cells[1] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 526u; }
    static int f2(int *p, int *q) { *p += 533; return *q * 2; }
    sum += f2(&cells[7], &cells[6]);
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f3(int *p, int *q) { *p += 318; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    cells[2] = cells[1] + 610;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[3] = cells[3] + 524;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[2 % 6] = 347; cells[3] += arr[2 % 6];
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }