static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[6] + 280;
    static int f1(int *p, int *q) { *p += 386; return *q * 2; }
    sum += f1(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[4 % 6] = 263; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f2(int *p, int *q) { *p += 190; return *q * 2; }
    sum += f2(&cells[2], &cells[7]);
    static int f3(int *p, int *q) { *p += 93; return *q * 2; }
    sum += f3(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 498; cells[1] += arr[0 % 6];
    arr[2 % 6] = 811; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[2] + 994;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    static int f4(int *p, int *q) { *p += 931; return *q * 2; }
    sum += f4(&cells[1], &cells[6]);
    arr[3 % 6] = 222; cells[5] += arr[3 % 6];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 824u; }
    static int f5(int *p, int *q) { *p += 284; return *q * 2; }
    sum += f5(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }